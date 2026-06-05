
use crate::prelude::*;


#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum CharKind {
  Char,
  BChar,
}


impl CharKind {
  pub(crate) const CHAR_PREFIX: &[u8]=b"'";
  pub(crate) const BCHAR_PREFIX: &[u8]=b"b'";

  pub(crate) const SUFFIX: &[u8]=b"'";

  #[inline(always)]
  pub const fn prefix(&self)-> &[u8] {
    match self {
      Self::Char=> Self::CHAR_PREFIX,
      Self::BChar=> Self::BCHAR_PREFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    match self {
      Self::Char|Self::BChar=> Self::SUFFIX,
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

  #[inline]
  fn try_parse(self,buf: &[u8])-> Result<TokenKind,ParseCharErr> {
    let prefix=self.prefix();
    let suffix=self.suffix();
    let min_len=prefix.len() + suffix.len();

    if !buf.starts_with(prefix) || !buf.ends_with(suffix) || buf.len()<min_len {
      return Err(ParseCharErr::Unterminated);
    }

    let start=prefix.len();
    let end=buf.len() - suffix.len();
    let unquoted=&buf[start..end];

    match unescape::unescape(std::str::from_utf8(unquoted)?) {
      None=> Err(ParseCharErr::InvalidEscapeSequence),
      Some(s) if s.len()>1 => Err(ParseCharErr::TooLong),
      Some(_)=> Ok(TokenKind::Literal(LiteralKind::Char(self))),
    }
  }
}



#[inline(always)]
pub fn parse(buf: &[u8],kind: CharKind)-> TokenKind {
  match kind.try_parse(buf) {
    Ok(token_kind)=> token_kind,
    Err(err)=> TokenKind::Illegal(Reason::ParseCharErr(err)),
  }
}














