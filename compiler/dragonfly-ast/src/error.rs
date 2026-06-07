
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

#[derive(Debug,Clone)]
pub enum ErrorKind {
  UnclosedDelimiter(char),
  UnexpectedDelimiter(char),
  MismatchedDelimiter(char),
  Unexpected(String), // Not recommended to construct directly, subject to change.
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
  pub fn unexpected<S: ToString>(s: S)-> Self {
    Self::Unexpected(s.to_string())
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    match &self.kind {
      ErrorKind::UnclosedDelimiter(deli)=> write!(f,"unclosed delimiter {deli}")?,
      ErrorKind::UnexpectedDelimiter(deli)=> write!(f,"unexpected delimiter {deli}")?,
      ErrorKind::MismatchedDelimiter(deli)=> write!(f,"mismatched delimiter {deli}")?,
      ErrorKind::Unexpected(msg)=> write!(f,"{msg}")?,
    }

    write!(f," at {}",self.span)?;
    Ok(())
  }
}



