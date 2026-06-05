
pub use lexical_core::Error as ParseNumberErr;

pub use std::{
  str::Utf8Error,
  fmt::{
    self,
    Debug,
    Formatter,
  },
};



#[derive(Clone,PartialEq,Eq)]
pub enum Reason {
  ParseCharErr(ParseCharErr),
  ParseIdentErr(ParseIdentErr),
  ParseStringErr(ParseStringErr),
  ParseNumberErr(ParseNumberErr),
  ParseCommentErr(ParseCommentErr),
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ParseCommentErr {
  UnclosedDelimiter,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ParseStringErr {
  Unterminated,
  Utf8Error(Utf8Error),
  InvalidEscapeSequence,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ParseCharErr {
  TooLong,
  Unterminated,
  Utf8Error(Utf8Error),
  InvalidEscapeSequence,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ParseIdentErr {
  offset: usize,
  utf8_err: Option<Utf8Error>,
}


impl ParseIdentErr {
  #[inline(always)]
  pub fn new(offset: usize)-> Self {
    Self {
      offset,
      utf8_err: None,
    }
  }

  pub fn from_utf8_err(utf8_err: Utf8Error)-> Self {
    Self {
      utf8_err: Some(utf8_err),
      offset: utf8_err.valid_up_to()+1,
    }
  }
}

impl From<Utf8Error> for ParseStringErr {
  #[inline(always)]
  fn from(err: Utf8Error)-> Self {
    Self::Utf8Error(err)
  }
}

impl From<Utf8Error> for ParseCharErr {
  #[inline(always)]
  fn from(err: Utf8Error)-> Self {
    Self::Utf8Error(err)
  }
}


impl Debug for Reason {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    match self {
      Self::ParseCharErr(err)=> Debug::fmt(&err,f),
      Self::ParseIdentErr(err)=> Debug::fmt(&err,f),
      Self::ParseStringErr(err)=> Debug::fmt(&err,f),
      Self::ParseNumberErr(err)=> Debug::fmt(&err,f),
      Self::ParseCommentErr(err)=> Debug::fmt(&err,f),
    }
  }
}






