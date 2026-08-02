
mod guard;

use dragonfly_stable_hash::StableHash;
pub use guard::{
  SourceMapReadGuard,
  SourceMapWriteGuard,
};

use std::{
  cmp::Ordering,
  marker::{
    PhantomData,
    StructuralPartialEq,
  },
  sync::{
    RwLock,
    PoisonError,
    TryLockError,
  },
};





pub struct SourceMap {
  source_map: RwLock<Vec<Source>>,
  _marker: PhantomData<SourceId>,
}

pub struct Source {
  #[allow(unused)]
  pub(crate) buf: String,
  pub(crate) overflowing_len_map: Vec<u32>,
}

#[derive(Clone,Copy,Hash,StableHash,Debug)]
pub struct SourceId(pub(crate) u16);

#[derive(Clone,Copy,Hash,StableHash,Debug)]
pub struct LenId(u16);


impl SourceMap {
  #[inline]
  pub const fn new()-> Self {
    Self {
      source_map: RwLock::new(Vec::new()),
      _marker: PhantomData,
    }
  }

  #[inline]
  pub fn insert_source(&self,src: Source)-> SourceId {
    let mut source_map=self.source_map.write()
    .expect("failed to write to source map");
    let src_id=SourceId::from_idx(source_map.len());

    // this should never fail since src_id is always <= 0x7fff
    source_map.push(src);
    src_id
  }

  #[inline(always)]
  pub fn read(&self)-> Result<SourceMapReadGuard<'_>,PoisonError<SourceMapReadGuard<'_>>> {
    match self.source_map.read() {
      Ok(guard)=> Ok(SourceMapReadGuard(guard)),
      Err(err)=> Err(guard::convert_poison_err(err))
    }
  }

  #[inline(always)]
  pub fn write(&self)-> Result<SourceMapWriteGuard<'_>,PoisonError<SourceMapWriteGuard<'_>>> {
    match self.source_map.write() {
      Ok(guard)=> Ok(SourceMapWriteGuard(guard)),
      Err(err)=> Err(guard::convert_poison_err(err)),
    }
  }

  #[inline(always)]
  pub fn try_read(&self)-> Result<SourceMapReadGuard<'_>,TryLockError<SourceMapReadGuard<'_>>> {
    match self.source_map.try_read() {
      Ok(guard)=> Ok(SourceMapReadGuard(guard)),
      Err(err)=> Err(guard::convert_try_lock_err(err))
    }
  }

  #[inline(always)]
  pub fn try_write(&self)-> Result<SourceMapWriteGuard<'_>,TryLockError<SourceMapWriteGuard<'_>>> {
    match self.source_map.try_write() {
      Ok(guard)=> Ok(SourceMapWriteGuard(guard)),
      Err(err)=> Err(guard::convert_try_lock_err(err))
    }
  }
}



impl Source {
  #[inline(always)]
  pub const fn new(buf: String)-> Self {
    Self {
      buf,
      overflowing_len_map: vec![],
    }
  }

  #[inline]
  pub fn get_overflowing_len(&self,len_id: LenId)-> u32 {
    self.overflowing_len_map.get(len_id.as_usize())
    .copied()
    .expect("invalid len id. LenId is never supposed to be constructed manually.")
  }

  #[inline]
  pub fn insert_oveflowing_len(&mut self,len: u32)-> LenId {
    let len_id=LenId::from_usize(self.overflowing_len_map.len());

    // this should never fail since len_id is always <= 0x7fff
    self.overflowing_len_map.push(len);
    len_id
  }
}






impl SourceId {
  pub const DUMMY: Self=Self(0);
  const THREASHOLD: u16=0xffff;
  pub const MAX: Self=Self(Self::THREASHOLD-1);

  #[inline]
  const fn from_idx(id: usize)-> Self {
    assert!(id<Self::THREASHOLD as usize);
    Self(id as u16 + 1)
  }

  #[inline(always)]
  pub const fn dummy()-> Self {
    Self::DUMMY
  }

  #[inline(always)]
  pub const fn is_dummy(self)-> bool {
    Self::DUMMY==self
  }

  #[inline]
  pub const fn as_idx(self)-> usize {
    assert!(!self.is_dummy(),"invalid source id");
    self.0 as usize - 1
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



impl LenId {
  const MAX_VAL: u16=u16::MAX >> 1;

  #[inline]
  pub(crate) const fn new(len_id: u16)-> Self {
    assert!(len_id<=Self::MAX_VAL);
    Self(len_id)
  }

  #[inline]
  const fn from_usize(len_id: usize)-> Self {
    assert!(len_id<=Self::MAX_VAL as usize);
    Self(len_id as u16)
  }

  #[inline(always)]
  pub(crate) const fn as_u16(self)-> u16 {
    self.0 as _
  }

  #[inline(always)]
  pub const fn as_usize(self)-> usize {
    self.as_u16() as _
  }
}


impl const PartialOrd for LenId {
  #[inline(always)]
  fn partial_cmp(&self,other: &Self)-> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl const Ord for LenId {
  #[inline(always)]
  fn cmp(&self,other: &Self)-> Ordering {
    self.0.cmp(&other.0)
  }
}

impl const PartialEq for LenId {
  #[inline(always)]
  fn eq(&self,other: &Self)-> bool {
    self.0==other.0
  }
}

impl const Eq for LenId {}
impl StructuralPartialEq for LenId {}



