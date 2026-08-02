

use dragonfly_stable_hash::StableHash;
use dragonfly_data_structures::monotonic::MonotonicVec;

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
  fmt::{
    self,
    Debug,
    Formatter,
  },
};





pub struct SourceMap {
  source_map: Vec<Source>,
  _marker: PhantomData<Vec<(SourceId,Source)>>,
}

pub struct Source {
  #[allow(unused)]
  pub(crate) buf: Vec<u8>,
  pub(crate) overflowing_len_map: MonotonicVec<u32>,
}

#[derive(Clone,Copy,Hash,StableHash)]
pub struct SourceId(pub(crate) u16);

#[derive(Clone,Copy,Hash,StableHash)]
pub struct LenId(u16);


impl SourceMap {
  #[inline]
  pub const fn new()-> Self {
    Self {
      source_map: Vec::new(),
      _marker: PhantomData,
    }
  }

  #[inline(always)]
  pub const fn get(&self,src_id: SourceId)-> Option<&Source> {
    self.source_map.get(src_id.as_idx())
  }

  #[inline(always)]
  pub const fn get_mut(&mut self,src_id: SourceId)-> Option<&mut Source> {
    self.source_map.get_mut(src_id.as_idx())
  }

  #[inline(always)]
  pub fn len(&self)-> usize {
    self.source_map.len()
  }

  #[inline(always)]
  pub fn capacity(&self)-> usize {
    self.source_map.capacity()
  }

  #[inline]
  pub fn insert_source(&mut self,src: Source)-> SourceId {
    let src_id=SourceId::from_idx(self.source_map.len());

    // this should never fail since src_id is always <= 0x7fff
    self.source_map.push(src);
    src_id
  }
}

impl const Index<SourceId> for SourceMap {
  type Output=Source;
  #[inline]
  fn index(&self,src_id: SourceId)-> &Self::Output {
    &self.source_map[src_id.as_idx()]
  }
}

impl const IndexMut<SourceId> for SourceMap {
  #[inline]
  fn index_mut(&mut self,src_id: SourceId)-> &mut Self::Output {
    &mut self.source_map[src_id.as_idx()]
  }
}



impl Source {
  #[inline(always)]
  pub const fn new(buf: Vec<u8>)-> Self {
    Self {
      buf,
      overflowing_len_map: MonotonicVec::new(),
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

impl Debug for SourceId {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    let mut fmt=f.debug_tuple(stringify!(SourceId));

    match *self {
      Self::DUMMY=> fmt.field(&"<dummy>"),
      Self(id)=> fmt.field(&id),
    };

    fmt.finish()
  }
}



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



