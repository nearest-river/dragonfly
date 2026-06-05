
mod token_kind;

use crate::prelude::*;
pub use token_kind::TokenKind;

use std::fmt::{
  self,
  Debug,
  Formatter,
};


pub struct Token<'a> {
  pub span: Span,
  pub kind: TokenKind,
  pub repr: &'a [u8],
  pub(crate) _marker: ProcMacroAutoTraits,
}



impl Debug for Token<'_> {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    Debug::fmt(&self.kind,f)?;
    let repr=str::from_utf8(self.repr).unwrap();

    if !f.alternate() {
      let Span { hi, lo }=self.span;
      return write!(f,"({lo}..{hi})");
    }

    if let TokenKind::Literal(LiteralKind::Str(_)|LiteralKind::Char(_))=&self.kind {
      write!(f,"({repr})")
    } else {
      write!(f,"({repr:#?})")
    }
  }
}








