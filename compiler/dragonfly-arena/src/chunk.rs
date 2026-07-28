

use std::{
  ptr::{
    self,
    NonNull,
  },
  mem::{
    self,
    MaybeUninit,
  },
};




pub(crate) struct ArenaChunk<T=u8> {
  /// The raw storage for the arena chunk.
  pub(crate) storage: NonNull<[MaybeUninit<T>]>,
  /// The number of valid entries in the chunk.
  pub(crate) entries: usize,
}



impl<T> ArenaChunk<T> {
  #[inline]
  pub(crate) unsafe fn new(capacity: usize)-> Self {
    Self {
      entries: 0,
      storage: Box::leak(Box::new_uninit_slice(capacity)).into()
    }
  }

  #[inline]
  pub(crate) unsafe fn destroy(&mut self,len: usize) {
    // The branch on needs_drop() is an -O1 performance optimization.
    // Without the branch, dropping TypedArena<T> takes linear time.
    if !mem::needs_drop::<T>() {
      return;
    }

    // SAFETY: The caller must ensure that `len` elements of this chunk have been initialized.
    unsafe {
      let slice=self.storage.as_mut();
      slice[..len].assume_init_drop();
    }
  }

  #[inline]
  pub(crate) fn start(&mut self)-> *mut T {
    self.storage.as_ptr() as *mut T
  }

  #[inline]
  pub(crate) fn end(&mut self)-> *mut T {
    unsafe {
      if mem::size_of::<T>()==0 {
        ptr::without_provenance_mut(!0)
      } else {
        self.start().add(self.storage.len())
      }
    }
  }

}


unsafe impl<#[may_dangle] T> Drop for ArenaChunk<T> {
  fn drop(&mut self) {
    unsafe { drop(Box::from_raw(self.storage.as_mut())) }
  }
}



