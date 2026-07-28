
use smallvec::SmallVec;
use crate::chunk::ArenaChunk;
use std::{
  cmp,
  mem,
  ptr,
  slice,
  convert::Infallible,
  marker::PhantomData,
  cell::{
    Cell,
    RefCell,
  },
};


pub struct TypedArena<T> {
  /// A pointer to the next object to be allocated.
  ptr: Cell<*mut T>,
  /// A pointer to the end of the allocated area. When this pointer is
  /// reached, a new chunk is allocated.
  end: Cell<*mut T>,
  /// A vector of arena chunks.
  pub(crate) chunks: RefCell<Vec<ArenaChunk<T>>>,
  /// Marker indicating that dropping the arena causes its owned
  /// instances of `T` to be dropped.
  _own: PhantomData<T>,
}


impl<T> TypedArena<T> {
  #[inline]
  pub fn alloc(&self,object: T)-> &mut T {
    assert!(mem::size_of::<T>()!=0);

    if self.ptr==self.end {
      self.grow(1);
    }

    unsafe {
      let ptr=self.ptr.get();
      self.ptr.set(self.ptr.get().add(1));
      ptr::write(ptr,object);
      &mut *ptr
    }
  }

  #[inline]
  fn can_allocate(&self,additional: usize)-> bool {
    let delta=unsafe {
      self.end.get().offset_from_unsigned(self.ptr.get())
    };

    delta>=additional
  }

  /// Allocates storage for `len >= 1` values in this arena, and returns a
  /// raw pointer to the first value's storage.
  unsafe fn alloc_raw_slice(&self,len: usize)-> *mut T {
    assert!(mem::size_of::<T>()!=0);
    assert!(len!=0);

    if !self.can_allocate(len) {
      self.grow(len);
      debug_assert!(self.can_allocate(len));
    }

    let start_ptr=self.ptr.get();

    // SAFETY: `can_allocate`/`grow` ensures that there is enough space for `len` elements.
    unsafe {
      self.ptr.set(start_ptr.add(len));
    }

    start_ptr
  }

  /// Allocates the elements of this iterator into a contiguous slice in the `TypedArena`.
  ///
  /// Note: for reasons of reentrancy and panic safety we collect into a `SmallVec<[_; 8]>` before
  /// storing the elements in the arena.
  #[inline]
  pub fn alloc_from_iter<I: IntoIterator<Item=T>>(&self,iter: I)-> &mut [T] {
    self.try_alloc_from_iter(iter.into_iter().map(Ok::<T,Infallible>))
    .unwrap()
  }

  #[inline]
  pub fn try_alloc_from_iter<E>(&self,iter: impl IntoIterator<Item=Result<T,E>>)-> Result<&mut [T],E> {
    assert!(mem::size_of::<T>()!=0);

    let mut vec=iter.into_iter()
    .collect::<Result<SmallVec<[T;8]>,E>>()?;
    if vec.is_empty() {
      return Ok(&mut []);
    }

    // Move the content to the arena by copying and then forgetting it.
    let len=vec.len();

    // SAFETY: After allocating raw storage for exactly `len` values, we
    // must fully initialize the storage without panicking, and we must
    // also prevent the stale values in the vec from being dropped.
    Ok(unsafe {
      let start_ptr = self.alloc_raw_slice(len);
      // Initialize the newly-allocated storage without panicking.
      vec.as_ptr().copy_to_nonoverlapping(start_ptr, len);
      // Prevent the stale values in the vec from being dropped.
      vec.set_len(0);
      slice::from_raw_parts_mut(start_ptr, len)
    })
  }



  #[cold]
  #[inline(never)]
  pub fn grow(&self,additional: usize) {
    unsafe {
      // We need the element size to convert chunk sizes (ranging from
      // PAGE to HUGE_PAGE bytes) to element counts.
      let elem_size=mem::size_of::<T>().max(1);
      let mut chunks=self.chunks.borrow_mut();

      let mut new_cap;
      if let Some(last_chunk)=chunks.last_mut() {
        // If a type is `!needs_drop`, we don't need to keep track of how many elements
        // the chunk stores - the field will be ignored anyway.
        if mem::needs_drop::<T>() {
          // SAFETY: trust me bro.
          last_chunk.entries=self.ptr.get().offset_from_unsigned(last_chunk.start());
        }

        // If the previous chunk's len is less than HUGE_PAGE
        // bytes, then this chunk will be least double the previous
        // chunk's size.
        new_cap=cmp::min(last_chunk.storage.len(),crate::HUGE_PAGE_SIZE/elem_size/2);
        new_cap*=2;
      } else {
        new_cap=crate::PAGE_SIZE/elem_size;
      };

      new_cap=cmp::max(new_cap,additional);

      let chunk=chunks.push_mut(ArenaChunk::<T>::new(new_cap));
      self.ptr.set(chunk.start());
      self.end.set(chunk.end());
    }
  }

  /// Drops the contents of the last chunk. The last chunk is partially empty, unlike all other chunks.
  pub(crate) fn clear_last_chunk(&self,last_chunk: &mut ArenaChunk<T>) {
    let start=last_chunk.start().addr();
    let end=self.ptr.get().addr();

    assert!(mem::size_of::<T>()!=0);

    let delta=(end-start)/mem::size_of::<T>();

    unsafe {
      last_chunk.destroy(delta);
    }

    self.ptr.set(last_chunk.start());
  }

}

unsafe impl<#[may_dangle] T> Drop for TypedArena<T> {
  fn drop(&mut self) {
    unsafe {
      let mut chunks_borrow=self.chunks.borrow_mut();
      if let Some(mut last_chunk)=chunks_borrow.pop() {
        self.clear_last_chunk(&mut last_chunk);

        for chunk in chunks_borrow.iter_mut() {
          chunk.destroy(chunk.entries);
        }
      }
      // Box handles deallocation of `last_chunk` and `self.chunks`.
    }
  }
}

unsafe impl<T: Send> Send for TypedArena<T> {}

impl<T> Default for TypedArena<T> {
  /// Creates a new `TypedArena`.
  fn default()-> TypedArena<T> {
    TypedArena {
      // We set both `ptr` and `end` to 0 so that the first call to
      // alloc() will trigger a grow().
      ptr: Cell::new(ptr::null_mut()),
      end: Cell::new(ptr::null_mut()),
      chunks: Default::default(),
      _own: PhantomData,
    }
  }
}



