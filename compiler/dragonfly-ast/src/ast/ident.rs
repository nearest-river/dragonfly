
use crate::prelude::*;

#[derive(Debug,Clone)]
pub struct Ident {
  pub(crate) span: Span,
  pub(crate) is_raw: bool,
}


impl Ident {
  pub(crate) fn new(span: Span,is_raw: bool)-> Self {
    Self {
      span,
      is_raw,
    }
  }

  pub fn parse(tt: TokenTree)-> Result<Self> {
    let span=tt.span();
    let token=match tt {
      TokenTree::Token(token)=> token,
      TokenTree::Group(group)=> {
        let kind=ErrorKind::unexpected(format!("expected identifier found {:#?}",group));
        return Err(Error::new(kind,span));
      },
    };

    match token.kind {
      TokenKind::Ident=> Ok(Self::new(span,false)),
      TokenKind::RawIdent=> Ok(Self::new(span,true)),
      token=> {
        let kind=ErrorKind::unexpected(format!("expected identifier found {:#?}",token));
        return Err(Error::new(kind,span));
      },
    }
  }
}



