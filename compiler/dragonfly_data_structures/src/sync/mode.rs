
use crate::marker::*;
use std::{
  ops::{
    Deref,
    DerefMut,
  },
  sync::atomic::{
    AtomicU8,
    Ordering,
  },
};

const UNINITIALIZED: u8=0;
const DYN_NOT_THREAD_SAFE: u8=1;
const DYN_THREAD_SAFE: u8=2;

static DYN_THREAD_SAFE_MODE: AtomicU8=AtomicU8::new(UNINITIALIZED);

#[derive(Copy,Clone)]
pub struct FromDyn<T>(T);


impl<T> FromDyn<T> {
  #[inline(always)]
  pub fn derive<O>(&self, val: O) -> FromDyn<O> {
    // We already did the check for `sync::is_dyn_thread_safe()` when creating `Self`
    FromDyn(val)
  }

  #[inline(always)]
  pub fn into_inner(self) -> T {
    self.0
  }
}

// `FromDyn` is `Send` if `T` is `DynSend`, since it ensures that sync::is_dyn_thread_safe() is true.
unsafe impl<T: DynSend> Send for FromDyn<T> {}

// `FromDyn` is `Sync` if `T` is `DynSync`, since it ensures that sync::is_dyn_thread_safe() is true.
unsafe impl<T: DynSync> Sync for FromDyn<T> {}

impl<T> Deref for FromDyn<T> {
  type Target=T;

  #[inline(always)]
  fn deref(&self)-> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for FromDyn<T> {
  #[inline(always)]
  fn deref_mut(&mut self)-> &mut Self::Target {
    &mut self.0
  }
}

#[inline(always)]
pub fn check_dyn_thread_safe()-> Option<FromDyn<()>> {
  is_dyn_thread_safe()
  .then_some(FromDyn(()))
}

#[inline]
pub fn is_dyn_thread_safe()-> bool {
  match DYN_THREAD_SAFE_MODE.load(Ordering::Relaxed) {
    DYN_NOT_THREAD_SAFE=> false,
    DYN_THREAD_SAFE=> true,
    _=> panic!("uninitialized dyn_thread_safe mode!"),
  }
}

#[inline]
pub(super) fn might_be_dyn_thread_safe()-> bool {
  DYN_THREAD_SAFE_MODE.load(Ordering::Relaxed) != DYN_NOT_THREAD_SAFE
}

#[inline]
pub fn set_dyn_thread_safe_mode(mode: bool) {
  let set=if mode { DYN_THREAD_SAFE } else { DYN_NOT_THREAD_SAFE };
  let prev=DYN_THREAD_SAFE_MODE.compare_exchange(
    UNINITIALIZED,
    set,
    Ordering::Relaxed,
    Ordering::Relaxed,
  );

  assert!(prev.is_ok() || prev==Err(set));
}




