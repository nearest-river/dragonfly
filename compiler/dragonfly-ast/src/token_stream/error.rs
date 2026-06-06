
use crate::prelude::*;
use std::fmt::{
  self,
  Display,
  Formatter,
};



#[derive(Debug,thiserror::Error)]
pub struct Error {
  span: Span,
  kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
  UnclosedDelimiter(Delimiter),
  UnexpectedClosingDelimiter(Delimiter),
  MismatchedClosingDelimiter(Delimiter),
}

impl Error {
  #[inline(always)]
  pub const fn new(kind: ErrorKind,span: Span)-> Self {
    Self {
      span,
      kind,
    }
  }
}


impl ErrorKind {
  #[inline(always)]
  pub const fn as_str(&self)-> &'static str {
    match self {
      Self::UnclosedDelimiter(_)=> "unclosed delimiter",
      Self::UnexpectedClosingDelimiter(_)=> "unexpected closing delimiter",
      Self::MismatchedClosingDelimiter(_)=> "mismatched closing delimiter",
    }
  }

  #[inline]
  pub const fn hint(&self)-> Option<char> {
    let hint=match self {
      Self::UnclosedDelimiter(deli)=> deli.as_chars().0,
      Self::MismatchedClosingDelimiter(deli)=> deli.as_chars().1,
      Self::UnexpectedClosingDelimiter(deli)=> deli.as_chars().1,
    };

    Some(hint)
  }
}


impl Display for Error {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    let kind=self.kind.as_str();
    let span=self.span;
    match self.kind.hint() {
      Some(hint)=> write!(f,"{kind} `{hint}` between {span}"),
      None=> write!(f,"{kind} between {span}"),
    }
  }
}


