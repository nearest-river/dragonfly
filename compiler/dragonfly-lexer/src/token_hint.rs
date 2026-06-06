
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
  RawLifetime,
  Lifetime,
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
}
