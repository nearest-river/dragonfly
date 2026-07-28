

use smallvec::SmallVec;

use crate::chunk::ArenaChunk;
use std::{
  cmp,
  mem,
  ptr,
  hint,
  slice,
  alloc::Layout,
  convert::Infallible,
  cell::{
    Cell,
    RefCell,
  },
};



pub struct DroplessArena {
  start: Cell<*mut u8>,
  end: Cell<*mut u8>,
  chunks: RefCell<Vec<ArenaChunk>>,
}

const DROPLESS_ALIGN: usize=mem::align_of::<usize>();


impl DroplessArena {
  #[cold]
  #[inline(never)]
  fn grow(&self,layout: Layout) {
    // Add some padding so we can align `self.end` while
    // still fitting in a `layout` allocation.
    let additional=layout.size() + cmp::max(DROPLESS_ALIGN,layout.align()) - 1;

    unsafe {
      let mut chunks=self.chunks.borrow_mut();
      let new_cap=if let Some(last_chunk)=chunks.last_mut() {
        let cap=last_chunk.storage.len().min(crate::HUGE_PAGE_SIZE / 2);
        cap*2
      } else {
        crate::PAGE_SIZE
      };
      let new_cap=cmp::max(additional,new_cap);
      let chunk=chunks.push_mut(ArenaChunk::new(align_up(new_cap,crate::PAGE_SIZE)));
      self.start.set(chunk.start());

      let end=align_down(chunk.end().addr(),DROPLESS_ALIGN);

      debug_assert!(chunk.start().addr() <= end);
      self.end.set(chunk.end().with_addr(end));
    }
  }

  #[inline]
  pub fn alloc_raw(&self,layout: Layout)-> *mut u8 {
    assert!(layout.size()!=0);

    // This loop executes once or twice: if allocation fails the first
    // time, the `grow` ensures it will succeed the second time.
    loop {
      let start=self.start.get().addr();
      let old_end=self.end.get();
      let end=old_end.addr();

      let bytes=align_up(layout.size(),DROPLESS_ALIGN);

      // Tell LLVM that `end` is aligned to DROPLESS_ALIGNMENT.
      unsafe {
        hint::assert_unchecked(end == align_down(end,DROPLESS_ALIGN))
      };

      if let Some(sub)=end.checked_sub(bytes) {
        let new_end=align_down(sub,layout.align());

        if start<=new_end {
          let new_end=old_end.with_addr(new_end);

          // `new_end` is aligned to DROPLESS_ALIGNMENT as `align_down`
          // preserves alignment as both `end` and `bytes` are already
          // aligned to DROPLESS_ALIGNMENT.
          self.end.set(new_end);
          return new_end;
        }
      }

      self.grow(layout);
    }
  }

  #[inline]
  pub fn alloc<T>(&self,object: T)-> &mut T {
    assert!(!mem::needs_drop::<T>());
    assert!(mem::size_of::<T>()!=0);

    let mem=self.alloc_raw(Layout::new::<T>()) as *mut T;
    unsafe {
      ptr::write(mem,object);
      &mut *mem
    }
  }

  #[inline]
  pub fn alloc_slice<T: Copy+Clone>(&self,slice: &[T])-> &mut [T] {
    assert!(!mem::needs_drop::<T>());
    assert!(mem::size_of::<T>()!=0);
    assert!(!slice.is_empty());

    let mem=self.alloc_raw(Layout::for_value::<[T]>(slice)) as *mut T;

    unsafe {
      mem.copy_from_nonoverlapping(slice.as_ptr(),slice.len());
      slice::from_raw_parts_mut(mem,slice.len())
    }
  }

  #[inline]
  pub fn alloc_str(&self,s: &str)-> &str {
    let slice=self.alloc_slice(s.as_bytes());
    // SAFETY: trust me bro.
    unsafe {
      str::from_utf8_unchecked(slice)
    }
  }

  /// # Safety
  ///
  /// The caller must ensure that `mem` is valid for writes up to `size_of::<T>() * len`, and that
  /// that memory stays allocated and not shared for the lifetime of `self`. This must hold even
  /// if `iter.next()` allocates onto `self`.
  #[inline]
  unsafe fn write_from_iter<T,I: Iterator<Item=T>>(&self,mut iter: I,len: usize,mem: *mut T)-> &mut [T] {
    let mut i=0;

    // Use a manual loop since LLVM manages to optimize it better for
    // slice iterators
    loop {
      unsafe {
        match iter.next() {
          Some(val) if i<len => mem.add(i).write(val),
          Some(_)|None => return slice::from_raw_parts_mut(mem,i),
        }
      }

      i+=1;
    }
  }

  pub fn alloc_from_iter<T,I: IntoIterator<Item=T>>(&self,iter: I)-> &mut [T] {
    // Warning: this function is reentrant: `iter` could hold a reference to `&self` and
    // allocate additional elements while we're iterating.
    let iter=iter.into_iter();

    assert!(mem::size_of::<T>()!=0);
    assert!(!mem::needs_drop::<T>());

    let size_hint=iter.size_hint();

    match size_hint {
      (min,Some(max)) if min==max => {
        let len=min;
        if len==0 {
          return &mut [];
        }

        let mem=self.alloc_raw(Layout::array::<T>(len).unwrap()) as *mut T;
        // SAFETY: `write_from_iter` doesn't touch `self`. It only touches the slice we just
        // reserved. If the iterator panics or doesn't output `len` elements, this will
        // leave some unallocated slots in the arena, which is fine because we do not call `drop`.
        unsafe {
          self.write_from_iter(iter, len, mem)
        }
      },
      (_,_)=> outline(move || self.try_alloc_from_iter(iter.map(Ok::<T,Infallible>)).unwrap()),
    }
  }



  #[inline]
  pub fn try_alloc_from_iter<T,E>(&self,iter: impl IntoIterator<Item=Result<T,E>>)-> Result<&mut [T], E> {
    assert!(mem::size_of::<T>()!=0);

    let mut vec=iter.into_iter().collect::<Result<SmallVec<[T;8]>,E>>()?;
    if vec.is_empty() {
      return Ok(&mut []);
    }

    // Move the content to the arena by copying and then forgetting it.
    let len=vec.len();
    Ok(unsafe {
      let start_ptr=self.alloc_raw(Layout::for_value::<[T]>(vec.as_slice())) as *mut T;

      vec.as_ptr()
      .copy_to_nonoverlapping(start_ptr,len);
      vec.set_len(0);

      slice::from_raw_parts_mut(start_ptr,len)
    })
  }
}


unsafe impl Send for DroplessArena {}

impl Default for DroplessArena {
  #[inline]
  fn default()-> Self {
    Self {
      start: Cell::new(ptr::null_mut()),
      end: Cell::new(ptr::null_mut()),
      chunks: RefCell::new(vec![])
    }
  }
}




/// This calls the passed function while ensuring it won't be inlined into the caller.
#[inline(never)]
#[cold]
fn outline<R,F: FnOnce()-> R>(f: F)-> R {
  f()
}


#[inline(always)]
fn align_down(val: usize, align: usize)-> usize {
  debug_assert!(align.is_power_of_two());
  val & !(align - 1)
}

#[inline(always)]
fn align_up(val: usize, align: usize)-> usize {
  debug_assert!(align.is_power_of_two());
  (val + align - 1) & !(align - 1)
}


