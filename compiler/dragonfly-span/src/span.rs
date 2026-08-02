
use dragonfly_stable_hash::StableHash;

use crate::source_map::{
  LenId,
  Source,
  SourceId,
};

use std::{
  hint,
  ops::Range,
  marker::StructuralPartialEq,
  cmp::{
    self,
    Ordering,
  },
  fmt::{
    self,
    Debug,
    Formatter,
  },
};

#[rustc_pass_by_value]
#[derive(Clone,Copy,StableHash,PartialEq,Eq)]
pub struct Span {
  idx: u32,
  len: Len,
  source_id: SourceId,
}

#[derive(Clone,Copy,Hash,StableHash)]
#[repr(transparent)]
struct Len(u16);


impl Span {
  pub const DUMMY: Self=Self::dummy();

  #[inline]
  pub const fn new(idx: u32,len: u16,source_id: SourceId)-> Self {
    let len=Len::new(len);
    Self {
      idx,
      len,
      source_id,
    }
  }

  #[inline(always)]
  pub const fn dummy()-> Self {
    Self {
      idx: 0,
      len: Len::DUMMY,
      source_id: SourceId::DUMMY,
    }
  }

  #[inline(always)]
  pub fn overflowing(idx: u32,len_id: LenId,source_id: SourceId)-> Self {
    Self {
      idx,
      source_id,
      len: Len::from_id(len_id),
    }
  }

  #[inline(always)]
  pub const fn is_dummy(self)-> bool {
    self.idx==0 && self.len.is_dummy() && self.source_id.is_dummy()
  }

  #[inline(always)]
  pub const fn is_overflowing(self)-> bool {
    self.len.is_overflowing()
  }

  #[inline(always)]
  pub fn byte_range(self)-> Range<usize> {
    let start=self.idx as usize;
    let len=if self.len.is_overflowing() {
      self.len.as_usize()
    } else {
      hint::cold_path();
      let len=crate::with_source_map(|source_map| {
        source_map[self.source_id].get_overflowing_len(self.len.as_len_id())
      });

      len as usize 
    };

    let end=start+len;

    start..end
  }

  #[inline(always)]
  pub const fn join_non_overflowing(self,other: Span)-> Span {
    assert!(self.source_id==other.source_id);
    debug_assert!(!self.is_overflowing() && !other.is_overflowing());

    let idx=cmp::min(self.idx,other.idx);
    let len=cmp::max(self.len,other.len);
    let source_id=self.source_id;

    Span {
      len,
      idx,
      source_id,
    }
  }

  #[inline]
  pub fn join_overflowing(self,other: Span)-> Span {
    assert!(self.source_id==other.source_id);
    let source_id=self.source_id;

    crate::with_source_map_mut(|source_map| {
      let src=&mut source_map[source_id];

      let idx=cmp::min(self.idx,other.idx);
      let len=cmp::max(self.len.load(src),other.len.load(src));
      let len_id=src.insert_oveflowing_len(len);

      Span::overflowing(idx,len_id,source_id)
    })
  }

  #[inline]
  pub fn join(self,other: Span)-> Span {
    if !self.is_overflowing() {
      self.join_non_overflowing(other)
    } else {
      hint::cold_path();
      self.join_overflowing(other)
    }
  }
}


impl Debug for Span {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    let alternate=f.alternate();
    let mut fmt=f.debug_tuple(stringify!(Span));

    fmt.field(&self.idx);
    fmt.field(&self.len);
    if !alternate {
      fmt.field(&self.source_id);
    }

    fmt.finish()
  }
}


impl Len {
  pub const DUMMY: Self=Len(0);
  const MAX_VAL: u16=u16::MAX >> 1;
  const OVERFLOW_MASK: u16=!Self::MAX_VAL;

  #[inline(always)]
  const fn new(len: u16)-> Self {
    assert!(len<=Self::MAX_VAL);
    Self(len)
  }

  #[inline(always)]
  const fn from_id(len_id: LenId)-> Self {
    // SAFETY(nate): construction of LenId is always safe
    Self(Self::OVERFLOW_MASK|len_id.as_u16())
  }

  #[inline(always)]
  const fn is_dummy(self)-> bool {
    Self::DUMMY==self
  }

  #[inline(always)]
  const fn is_overflowing(self)-> bool {
    self.0 & Self::OVERFLOW_MASK != 0
  }

  #[inline(always)]
  const fn is_inline(self)-> bool {
    !self.is_overflowing()
  }

  #[inline(always)]
  const fn as_u16(self)-> u16 {
    assert!(self.is_inline());
    self.0 as u16
  }

  #[inline(always)]
  const fn as_u32(self)-> u32 {
    self.as_u16() as _
  }

  #[inline(always)]
  const fn as_usize(self)-> usize {
    self.as_u16() as usize
  }

  #[inline]
  const fn as_len_id(self)-> LenId {
    assert!(self.is_overflowing());
    LenId::new(self.0 & !Self::OVERFLOW_MASK)
  }

  #[inline(always)]
  fn load(self,source: &Source)-> u32 {
    if self.is_inline() {
      return self.as_u32();
    }

    let len_id=self.as_len_id();

    source.get_overflowing_len(len_id)
  }
}

impl const PartialOrd for Len {
  #[inline(always)]
  fn partial_cmp(&self,other: &Self)-> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl const Ord for Len {
  #[inline(always)]
  fn cmp(&self,other: &Self)-> Ordering {
    self.0.cmp(&other.0)
  }
}

impl const PartialEq for Len {
  #[inline(always)]
  fn eq(&self,other: &Self)-> bool {
    self.0==other.0
  }
}

impl const Eq for Len {}
impl StructuralPartialEq for Len {}

impl Debug for Len {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    match *self {
      len if len.is_overflowing()=> write!(f,"<overflown>"),
      Self::DUMMY if f.alternate()=> write!(f,"<dummy>"),
      _=> write!(f,"{}",self.0),
    }
  }
}


