
use crate::prelude::*;

pub struct TokenHint {
  pub len: usize,
  pub hint: TokenHintKind,
}

#[derive(Debug)]
pub enum TokenHintKind {
  Float(Option<FloatKind>),
  Int(Option<IntKind>),
  Str(StrKind),
  Char(CharKind),
  Comment(CommentKind),
  Illegal(Reason),
  RIdent,
  Other,
}

impl TokenHint {
  #[inline(always)]
  pub const fn new(len: usize,hint: TokenHintKind)-> Self {
    Self {
      len,
      hint,
    }
  }
}

impl TokenHintKind {
  pub const INFERRED_FLOAT: Self=Self::Float(None);
  pub const INFERRED_INT: Self=Self::Int(None);

  #[inline]
  pub const fn suffix_size_hint(&self)-> Option<usize> {
    match self {
      Self::Float(Some(kind))=> Some(kind.suffix_len()),
      Self::Int(Some(kind))=> Some(kind.suffix_len()),
      Self::Comment(kind)=> Some(kind.suffix_len()),
      Self::Str(kind)=> Some(kind.suffix_len()),
      Self::Char(kind)=> Some(kind.suffix_len()),
      Self::Int(None)|Self::Float(None)=> None,
      Self::RIdent=> None,
      Self::Illegal(_)=> None,
      Self::Other=> None,
    }
  }
}
