
use super::{
  Source,
  SourceId,
};

use std::{
  ops::{
    Index,
    IndexMut,
  },
  sync::{
    PoisonError,
    TryLockError,
    RwLockReadGuard,
    RwLockWriteGuard,
  },
};



pub struct SourceMapReadGuard<'a>(pub(crate) RwLockReadGuard<'a,Vec<Source>>);
pub struct SourceMapWriteGuard<'a>(pub(crate) RwLockWriteGuard<'a,Vec<Source>>);


impl SourceMapReadGuard<'_> {
  #[inline(always)]
  pub fn get(&self,idx: SourceId)-> Option<&Source> {
    assert_ne!(idx,SourceId::DUMMY,"invalid source id");
    self.0.get(idx.as_usize())
  }

  #[inline(always)]
  pub fn len(&self)-> usize {
    self.0.len()
  }

  #[inline(always)]
  pub fn capacity(&self)-> usize {
    self.0.capacity()
  }
}

impl SourceMapWriteGuard<'_> {
  #[inline(always)]
  pub fn get(&self,idx: SourceId)-> Option<&Source> {
    self.0.get(idx.as_usize())
  }

  #[inline(always)]
  pub fn get_mut(&mut self,idx: SourceId)-> Option<&mut Source> {
    self.0.get_mut(idx.as_usize())
  }

  #[inline(always)]
  pub fn len(&self)-> usize {
    self.0.len()
  }

  #[inline(always)]
  pub fn capacity(&self)-> usize {
    self.0.capacity()
  }

  #[inline(always)]
  pub fn push(&mut self,source: Source) {
    self.0.push(source);
  }
}





impl Index<SourceId> for SourceMapReadGuard<'_> {
  type Output=Source;
  #[inline(always)]
  fn index(&self,idx: SourceId)-> &Self::Output {
    assert_ne!(idx,SourceId::DUMMY,"invalid source id");

    self.get(idx).expect("index out of bounds")
  }
}

impl Index<SourceId> for SourceMapWriteGuard<'_> {
  type Output=Source;
  #[inline(always)]
  fn index(&self,idx: SourceId)-> &Self::Output {
    assert_ne!(idx,SourceId::DUMMY,"invalid source id");

    self.get(idx).expect("index out of bounds")
  }
}

impl IndexMut<SourceId> for SourceMapWriteGuard<'_> {
  #[inline(always)]
  fn index_mut(&mut self,idx: SourceId)-> &mut Self::Output {
    assert_ne!(idx,SourceId::DUMMY,"invalid source id");

    self.get_mut(idx).expect("index out of bounds")
  }
}

impl<'a> From<RwLockReadGuard<'a,Vec<Source>>> for SourceMapReadGuard<'a> {
  #[inline(always)]
  fn from(value: RwLockReadGuard<'a,Vec<Source>>)-> Self {
    SourceMapReadGuard(value)
  }
}

impl<'a> From<RwLockWriteGuard<'a,Vec<Source>>> for SourceMapWriteGuard<'a> {
  #[inline(always)]
  fn from(value: RwLockWriteGuard<'a,Vec<Source>>)-> Self {
    SourceMapWriteGuard(value)
  }
}


#[inline(always)]
pub(crate) fn convert_poison_err<T,U: From<T>>(err: PoisonError<T>)-> PoisonError<U> {
  PoisonError::new(err.into_inner().into())
}

#[inline(always)]
pub(crate) fn convert_try_lock_err<T,U: From<T>>(err: TryLockError<T>)-> TryLockError<U> {
  match err {
    TryLockError::WouldBlock=> TryLockError::WouldBlock,
    TryLockError::Poisoned(err)=> TryLockError::Poisoned(convert_poison_err(err))
  }
}



