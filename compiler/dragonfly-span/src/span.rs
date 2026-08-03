
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
#[derive(Clone,Copy,StableHash)]
pub struct Span {
  pub(crate) idx: u32,
  pub(crate) len: Len,
  pub(crate) source_id: SourceId,
}

#[derive(Clone,Copy,Hash,StableHash)]
#[repr(transparent)]
pub(crate) struct Len(u16);


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
    self==Self::DUMMY
  }

  #[inline(always)]
  pub const fn is_partially_dummy(self)-> bool {
    self.len.is_dummy() || self.source_id.is_dummy()
  }

  #[inline(always)]
  pub const fn is_overflowing(self)-> bool {
    self.len.is_overflowing()
  }

  #[inline(always)]
  pub const fn is_inline(self)-> bool {
    self.len.is_inline()
  }

  #[inline(always)]
  pub fn byte_range(self)-> Range<usize> {
    let start=self.idx as usize;
    let len=if self.len.is_inline() {
      unsafe { self.len.as_inline_len_unchecked() as usize }
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

  #[inline]
  /// SAFETY: the caller must hold that both `self` and `other` are inline.
  pub const unsafe fn join_inline_unchecked(self,other: Span)-> Span {
    let idx=cmp::min(self.idx,other.idx);
    let len=cmp::max(self.len,other.len);
    let source_id=self.source_id;

    Span {
      len,
      idx,
      source_id,
    }
  }

  #[inline(always)]
  pub fn join(self,other: Span)-> Span {
    if !self.is_inline() {
      return unsafe { self.join_inline_unchecked(other) };
    }
    hint::cold_path();
    let source_id=self.source_id;
    assert!(source_id==other.source_id && !source_id.is_dummy());

    crate::with_source_map_mut(|source_map| {
      // SAFETY: trust me bro.
      let src=unsafe { source_map.get_unchecked_mut(source_id) };

      let idx=cmp::min(self.idx,other.idx);
      let len=cmp::max(self.len.load(src),other.len.load(src));
      let len_id=src.insert_oveflowing_len(len);

      Span::overflowing(idx,len_id,source_id)
    })
  }
}

impl const Eq for Span {}
impl StructuralPartialEq for Span {}
impl const PartialEq for Span {
  #[inline(always)]
  fn eq(&self,other: &Self)-> bool {
    self.idx==other.idx && self.len==other.len && self.source_id==other.source_id
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
  pub(crate) const fn new(len: u16)-> Self {
    assert!(len<=Self::MAX_VAL);
    unsafe {
      Self::new_unchecked(len)
    }
  }

  #[inline(always)]
  pub(crate) const unsafe fn new_unchecked(len: u16)-> Self {
    Self(len)
  }

  #[inline(always)]
  pub(crate) const fn from_id(len_id: LenId)-> Self {
    // SAFETY(nate): construction of LenId is always safe
    unsafe {
      Self::new_unchecked(Self::OVERFLOW_MASK|len_id.as_u16())
    }
  }

  #[inline(always)]
  pub(crate) const fn is_dummy(self)-> bool {
    Self::DUMMY==self
  }

  #[inline(always)]
  pub(crate) const fn is_overflowing(self)-> bool {
    self.0 & Self::OVERFLOW_MASK != 0
  }

  #[inline(always)]
  pub(crate) const fn is_inline(self)-> bool {
    !self.is_overflowing()
  }

  #[inline]
  /// SAFETY: the caller must hold that `self` is an inline `Len`.
  pub(crate) const unsafe fn as_inline_len_unchecked(self)-> u16 {
    self.0 as u16
  }

  #[inline]
  pub(crate) const fn as_len_id(self)-> LenId {
    assert!(self.is_overflowing());
    unsafe {
      self.as_len_id_unchecked()
    }
  }

  /// SAFETY: the caller must hold that `self` is a overflowing `Len`.
  pub(crate) const unsafe fn as_len_id_unchecked(self)-> LenId {
    LenId::new(self.0 & !Self::OVERFLOW_MASK)
  }

  #[inline(always)]
  pub(crate) fn load(self,source: &Source)-> u32 {
    if self.is_inline() {
      return unsafe { self.as_inline_len_unchecked() as u32 };
    }

    unsafe {
      self.load_outline_unchecked(source)
    }
  }

  #[inline(always)]
  /// SAFETY: the caller must hold that `self` is a overflowing `Len`.
  pub(crate) unsafe fn load_outline_unchecked(self,source: &Source)-> u32 {
    // SAFETY: held by the caller
    let len_id=unsafe { self.as_len_id_unchecked() };

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


