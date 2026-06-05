
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
    if f.alternate() {
      return write!(f,"({:#?})",str::from_utf8(self.repr).unwrap());
    }

    let Span { hi, lo }=self.span;
    write!(f,"({lo}..{hi})",)
  }
}








