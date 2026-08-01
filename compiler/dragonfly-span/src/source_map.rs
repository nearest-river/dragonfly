
use dragonfly_stable_hash::StableHash;
use dragonfly_data_structures::fx::FxHashMap;

use std::{
  cmp::Ordering,
  ops::{
    Index,
    IndexMut,
  },
  marker::{
    PhantomData,
    StructuralPartialEq,
  },
  sync::{
    RwLock,
    PoisonError,
    TryLockError,
    RwLockReadGuard,
    RwLockWriteGuard,
    atomic::{
      AtomicU16,
      Ordering as AtomicOrdering,
    },
  },
};





pub struct SourceMap {
  counter: AtomicU16,
  source_map: RwLock<Vec<Source>>,
  _marker: PhantomData<SourceId>,
}

pub struct Source {
  pub(crate) buf: String,
  pub(crate) overflowing_len_map: FxHashMap<u32,u32>,
}

#[derive(Clone,Copy,Hash,StableHash,Debug)]
pub struct SourceId(pub(crate) u16);

pub struct SourceMapReadGuard<'a>(RwLockReadGuard<'a,Vec<Source>>);
pub struct SourceMapWriteGuard<'a>(RwLockWriteGuard<'a,Vec<Source>>);



impl SourceMap {
  #[inline]
  pub const fn new()-> Self {
    Self {
      counter: AtomicU16::new(0),
      source_map: RwLock::new(Vec::new()),
      _marker: PhantomData,
    }
  }

  #[inline]
  pub fn gen_id(&self)-> SourceId {
    let id=self.counter.update(AtomicOrdering::Relaxed,AtomicOrdering::Relaxed,|x| {
      assert!(SourceId(x)!=SourceId::DUMMY,"maximum number of sources reached.");

      x+1
    });

    SourceId(id)
  }

  #[inline(always)]
  pub fn read(&self)-> Result<SourceMapReadGuard<'_>,PoisonError<SourceMapReadGuard<'_>>> {
    match self.source_map.read() {
      Ok(guard)=> Ok(SourceMapReadGuard(guard)),
      Err(err)=> Err(Self::convert_poison_err(err))
    }
  }

  #[inline(always)]
  pub fn write(&self)-> Result<SourceMapWriteGuard<'_>,PoisonError<SourceMapWriteGuard<'_>>> {
    match self.source_map.write() {
      Ok(guard)=> Ok(SourceMapWriteGuard(guard)),
      Err(err)=> Err(Self::convert_poison_err(err)),
    }
  }

  #[inline(always)]
  pub fn try_read(&self)-> Result<SourceMapReadGuard<'_>,TryLockError<SourceMapReadGuard<'_>>> {
    match self.source_map.try_read() {
      Ok(guard)=> Ok(SourceMapReadGuard(guard)),
      Err(err)=> Err(Self::convert_try_lock_err(err))
    }
  }

  #[inline(always)]
  pub fn try_write(&self)-> Result<SourceMapWriteGuard<'_>,TryLockError<SourceMapWriteGuard<'_>>> {
    match self.source_map.try_write() {
      Ok(guard)=> Ok(SourceMapWriteGuard(guard)),
      Err(err)=> Err(Self::convert_try_lock_err(err))
    }
  }

  #[inline(always)]
  fn convert_poison_err<T,U: From<T>>(err: PoisonError<T>)-> PoisonError<U> {
    PoisonError::new(err.into_inner().into())
  }

  #[inline(always)]
  fn convert_try_lock_err<T,U: From<T>>(err: TryLockError<T>)-> TryLockError<U> {
    match err {
      TryLockError::WouldBlock=> TryLockError::WouldBlock,
      TryLockError::Poisoned(err)=> TryLockError::Poisoned(Self::convert_poison_err(err)),
    }
  }
}

impl SourceMapReadGuard<'_> {
  #[inline(always)]
  pub fn get(&self,idx: SourceId)-> Option<&Source> {
    assert_ne!(idx,SourceId::DUMMY,"invalid source id");
    self.0.get(idx.as_usize())
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






impl SourceId {
  pub const DUMMY: Self=Self(0xffff);

  #[inline(always)]
  pub const fn new(id: u16)-> Self {
    assert!(id!=Self::DUMMY.0);
    Self(id)
  }

  #[inline(always)]
  pub const fn dummy()-> Self {
    Self::DUMMY
  }

  #[inline(always)]
  pub const fn is_dummy(self)-> bool {
    Self::DUMMY==self
  }

  #[inline(always)]
  pub const fn as_usize(self)-> usize {
    self.0 as usize
  }
}

impl const PartialOrd for SourceId {
  #[inline(always)]
  fn partial_cmp(&self,other: &Self)-> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl const Ord for SourceId {
  #[inline(always)]
  fn cmp(&self,other: &Self)-> Ordering {
    self.0.cmp(&other.0)
  }
}

impl const PartialEq for SourceId {
  #[inline(always)]
  fn eq(&self,other: &Self)-> bool {
    self.0==other.0
  }
}

impl const Eq for SourceId {}
impl StructuralPartialEq for SourceId {}


