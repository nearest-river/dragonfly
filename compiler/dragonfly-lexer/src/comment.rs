
use crate::prelude::*;



#[derive(Clone,Debug,PartialEq,Eq)]
/// Same conventions as rust.
pub enum CommentKind {
  Line,
  DocLine,
  Block,
  DocBlock,
}

impl CommentKind {
  #[inline]
  pub fn parse(repr: &[u8])-> Option<Self> {
    let kind=match repr {
      buf if buf.starts_with(b"/**")=> Self::DocBlock,
      buf if buf.starts_with(b"/*")=> Self::Block,
      buf if buf.starts_with(b"///")=> Self::DocLine,
      buf if buf.starts_with(b"//")=> Self::Line,
      _=> return None
    };

    Some(kind)
  }

  #[inline(always)]
  pub const fn prefix(&self)-> & [u8] {
    match self {
      Self::Line=> b"//",
      Self::DocLine=> b"///",
      Self::Block=> b"/*",
      Self::DocBlock=> b"/**",
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    match self {
      Self::Block|Self::DocBlock=> b"*/",
      Self::Line|Self::DocLine=> b"",
    }
  }

  #[inline(always)]
  pub const fn prefix_len(&self)-> usize {
    self.prefix().len()
  }

  #[inline(always)]
  pub const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }
}


pub fn parse(buf: &[u8],kind: CommentKind)-> TokenKind {
  if buf.starts_with(kind.prefix()) && buf.ends_with(kind.suffix()) {
    return TokenKind::Comment(kind)
  }


  let reason=Reason::ParseCommentErr(ParseCommentErr::UnclosedDelimiter);
  return TokenKind::Illegal(reason);
}








