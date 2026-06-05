
pub(crate) mod marker;

pub(crate) use marker::*;
pub(crate) use std::num::*;

#[allow(unused_imports)]
pub(crate) use crate::{
  error::*,
  lifetime,
  token_hint::*,
  util::{
    self,
    exts::*,
  },
  literal::{
    int::IntKind,
    string::StrKind,
    float::FloatKind,
    character::CharKind,
  },
};

pub use crate::{
  span::*,
  token::*,
  error::Reason,
  comment::CommentKind,
  literal::LiteralKind,
};







