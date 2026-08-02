
use std::ops::{
  Deref,
  DerefMut,
};

#[repr(transparent)]
#[derive(Default,Clone)]
pub struct MonotonicVec<T>(Vec<T>);

impl<T> MonotonicVec<T> {
  #[inline(always)]
  pub const fn new()-> Self {
    Self(Vec::new())
  }

  #[inline(always)]
  pub fn with_capacity(capacity: usize)-> Self {
    Self(Vec::with_capacity(capacity))
  }

  #[inline(always)]
  pub fn push(&mut self,value: T) {
    self.0.push(value);
  }

  #[inline(always)]
  pub fn reserve(&mut self,additional: usize) {
    self.0.reserve(additional);
  }
}


impl<T> Deref for MonotonicVec<T> {
  type Target=Vec<T>;

  #[inline(always)]
  fn deref(&self)-> &Self::Target {
    &self.0
  }
}

impl<T> !DerefMut for MonotonicVec<T> {}




