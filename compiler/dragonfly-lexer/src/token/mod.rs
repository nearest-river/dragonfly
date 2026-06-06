
mod token_kind;

use crate::prelude::*;
pub use token_kind::TokenKind;

use std::fmt::{
  self,
  Debug,
  Formatter,
};

#[derive(Clone)]
pub struct Token {
  pub span: Span,
  pub kind: TokenKind,
}

impl Eq for Token {}
impl PartialEq<Token> for Token {
  fn eq(&self,other: &Token)-> bool {
    self.kind==other.kind
  }
}

impl Debug for Token {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      return Debug::fmt(&self.kind,f);
    }

    let mut fmt=f.debug_struct(stringify!(Token));

    fmt.field("kind",&self.kind);
    fmt.field("span",&self.span);

    fmt.finish()
  }
}



