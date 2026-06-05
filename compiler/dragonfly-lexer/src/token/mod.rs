
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
    let repr=str::from_utf8(self.repr).unwrap();
    if matches!(self.kind,TokenKind::Illegal(_)) {
      write!(f,"[{:#?}] ",repr)?;
    }

    Debug::fmt(&self.kind,f)?;

    if !f.alternate() {
      let Span { hi, lo }=self.span;
      return write!(f,"({lo}..{hi})");
    }

    match &self.kind {
      TokenKind::Literal(LiteralKind::Str(_)|LiteralKind::Char(_))=> write!(f,"({repr})"),
      TokenKind::Illegal(_)=> Ok(()),
      _=> write!(f,"({repr:#?})"),
    }
  }
}








