
// This module has a very short name because it's used a lot.
/// This module contains all the defined non-keyword `Symbol`s.
///
/// Given that `sym` is imported, use them like `sym::symbol_name`.
/// For example `sym::rustfmt` or `sym::u8`.
pub mod sym;
pub mod interner;

use dragonfly_stable_hash::{
  StableHash,
  StableHasher,
  StableHashCtxt,
  cmp::StableCompare,
};

use std::{
  cmp::Ordering,
  fmt::{
    self,
    Debug,
    Display,
    Formatter,
  }
};




#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Symbol(SymbolIndex);

// Used within both `Symbol` and `ByteSymbol`.
dragonfly_index::newtype_index! {
  #[orderable]
  struct SymbolIndex {}
}

impl Symbol {
  pub const fn new(n: u32)-> Self {
    Symbol(SymbolIndex::from_u32(n))
  }

  pub fn intern(_s: &str)-> Self {
    unimplemented!()
  }

  pub fn as_str(&self)-> &str {
    unimplemented!()
  }

  pub fn as_u32(self)-> u32 {
    self.0.as_u32()
  }

  pub fn is_empty(self)-> bool {
    unimplemented!()
  }

  pub fn to_ident_string(self)-> String {
    unimplemented!()
  }

  pub fn find_similar(self,_candidates: &[Symbol])-> Option<(Symbol,bool)> {
    unimplemented!()
  }
}

// TODO(nate): self.as_str()
impl Debug for Symbol {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    Debug::fmt(&self.0,f)
  }
}

// TODO(nate): self.as_str()
impl Display for Symbol {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    Debug::fmt(self,f)
  }
}


// TODO(nate): self.as_str()
impl StableHash for Symbol {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.as_u32().stable_hash(hcx,hasher);
  }
}

impl StableCompare for Symbol {
  const CAN_USE_UNSTABLE_SORT: bool=true;

  fn stable_cmp(&self, other: &Self)-> Ordering {
    self.as_str().cmp(other.as_str())
  }
}














