
use crate::prelude::*;



#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
pub enum StrKind {
  Str,
  BStr,
  CStr,
  RStr,
  BRStr,
  CRStr,
}



impl StrKind {
  pub(crate) const STR_PREFIX: &[u8]=b"\"";
  pub(crate) const BSTR_PREFIX: &[u8]=b"b\"";
  pub(crate) const CSTR_PREFIX: &[u8]=b"c\"";
  pub(crate) const RSTR_PREFIX: &[u8]=b"r#\"";
  pub(crate) const BRSTR_PREFIX: &[u8]=b"br#\"";
  pub(crate) const CRSTR_PREFIX: &[u8]=b"cr#\"";

  pub(crate) const SUFFIX: &[u8]=b"\"";
  pub(crate) const RSUFFIX: &[u8]=b"\"#";

  #[inline(always)]
  pub const fn prefix(&self)-> &[u8] {
    match self {
      Self::Str=> Self::STR_PREFIX,
      Self::BStr=> Self::BSTR_PREFIX,
      Self::CStr=> Self::CSTR_PREFIX,
      Self::RStr=> Self::RSTR_PREFIX,
      Self::BRStr=> Self::BRSTR_PREFIX,
      Self::CRStr=> Self::CRSTR_PREFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    match self {
      Self::Str|Self::BStr|Self::CStr=> Self::SUFFIX,
      Self::RStr|Self::BRStr|Self::CRStr=> Self::RSUFFIX,
    }
  }

  #[inline(always)]
  pub const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }

  #[inline(always)]
  pub const fn prefix_len(&self)-> usize {
    self.prefix().len()
  }

  #[inline(always)]
  pub const fn from_prefix(pat: &[u8])-> Option<Self> {
    match pat {
      Self::STR_PREFIX=> Some(Self::Str),
      Self::BSTR_PREFIX=> Some(Self::BStr),
      Self::CSTR_PREFIX=> Some(Self::CStr),
      Self::RSTR_PREFIX=> Some(Self::RStr),
      Self::BRSTR_PREFIX=> Some(Self::BRStr),
      Self::CRSTR_PREFIX=> Some(Self::CRStr),
      _=> None,
    }
  }

  #[inline]
  fn try_parse(self,buf: &[u8])-> Result<TokenKind,ParseStringErr> {
    let prefix=self.prefix();
    let suffix=self.suffix();
    let min_len=prefix.len() + suffix.len();

    if !buf.starts_with(prefix) || !buf.ends_with(suffix) || buf.len()<min_len {
      return Err(ParseStringErr::Unterminated);
    }

    match unescape::unescape(std::str::from_utf8(buf)?) {
      Some(_)=> Ok(TokenKind::Literal(LiteralKind::Str(self))),
      None=> Err(ParseStringErr::InvalidEscapeSequence),
    }
  }
}


#[inline(always)]
pub fn parse(buf: &[u8],kind: StrKind)-> TokenKind {
  match kind.try_parse(buf) {
    Ok(token_kind)=> token_kind,
    Err(err)=> TokenKind::Illegal(Reason::ParseStringErr(err))
  }
}













