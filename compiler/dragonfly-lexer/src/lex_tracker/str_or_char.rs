
use crate::prelude::*;


pub struct StrOrCharTracker {
  len: usize,
  inner: Inner,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Inner {
  Str,
  BStr,
  CStr,
  RStr,
  BRStr,
  CRStr,
  Char,
  BChar,
}

impl StrOrCharTracker {
  pub fn try_start(buf: &[u8])-> Option<Self> {
    let inner=match buf {
      buf if buf.starts_with(StrKind::Str.prefix())=> Inner::Str,
      buf if buf.starts_with(StrKind::BStr.prefix())=> Inner::BStr,
      buf if buf.starts_with(StrKind::CStr.prefix())=> Inner::CStr,
      buf if buf.starts_with(StrKind::RStr.prefix())=> Inner::RStr,
      buf if buf.starts_with(StrKind::BRStr.prefix())=> Inner::BRStr,
      buf if buf.starts_with(StrKind::CRStr.prefix())=> Inner::CRStr,
      buf if buf.starts_with(CharKind::Char.prefix())=> Inner::Char,
      buf if buf.starts_with(CharKind::BChar.prefix())=> Inner::BChar,
      _=> return None,
    };

    let len=inner.prefix_len();

    Some(Self {
      len,
      inner,
    })
  }

  pub fn try_finish(&mut self,buf: &[u8])-> Option<TokenHint> {
    if buf.starts_with(self.inner.suffix()) {
      self.len+=self.suffix_len();
      return Some(TokenHint::new(self.len,self.hint_kind()));
    }

    self.len+=1;
    None
  }

  #[inline(always)]
  const fn hint_kind(&self)-> TokenHintKind {
    match self.inner {
      Inner::Str=> TokenHintKind::Str(StrKind::Str),
      Inner::BStr=> TokenHintKind::Str(StrKind::BStr),
      Inner::CStr=> TokenHintKind::Str(StrKind::CStr),
      Inner::RStr=> TokenHintKind::Str(StrKind::RStr),
      Inner::BRStr=> TokenHintKind::Str(StrKind::BRStr),
      Inner::CRStr=> TokenHintKind::Str(StrKind::CRStr),
      Inner::Char=> TokenHintKind::Char(CharKind::Char),
      Inner::BChar=> TokenHintKind::Char(CharKind::BChar),
    }
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    self.inner.suffix_len()
  }

  #[inline]
  pub const fn prefix_len(&self)-> usize {
    self.inner.prefix_len()
  }
}





impl Inner {
  #[inline(always)]
  const fn suffix(&self)-> &[u8] {
    match self {
      Self::Str=> StrKind::Str.suffix(),
      Self::BStr=> StrKind::BStr.suffix(),
      Self::CStr=> StrKind::CStr.suffix(),
      Self::RStr=> StrKind::RStr.suffix(),
      Self::BRStr=> StrKind::BRStr.suffix(),
      Self::CRStr=> StrKind::BRStr.suffix(),
      Self::Char=> CharKind::Char.suffix(),
      Self::BChar=> CharKind::BChar.suffix(),
    }
  }

  #[inline(always)]
  const fn prefix(&self)-> &[u8] {
    match self {
      Self::Str=> StrKind::Str.prefix(),
      Self::BStr=> StrKind::BStr.prefix(),
      Self::CStr=> StrKind::CStr.prefix(),
      Self::RStr=> StrKind::RStr.prefix(),
      Self::BRStr=> StrKind::BRStr.prefix(),
      Self::CRStr=> StrKind::CRStr.prefix(),
      Self::Char=> CharKind::Char.prefix(),
      Self::BChar=> CharKind::BChar.prefix(),
    }
  }

  #[inline(always)]
  const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }

  #[inline(always)]
  const fn prefix_len(&self)-> usize {
    self.prefix().len()
  }
}

/*
  pub fn try_finish(&mut self,buf: &[u8])-> Option<TokenHint> {
    let hint=self.hint_kind();
    self.len+=1;

    if self.escape_suffix {
      self.escape_suffix=false;
      return None;
    }

    let cond=match (self.inner,buf[0],buf.get(1).copied()) {
      (Inner::RStr,_,_) if buf.starts_with(StrKind::RSUFFIX)=> true,
      (Inner::Char|Inner::BChar,b'\n'|b'\r'|CharKind::SUFFIXB,_)=> true,
      (Inner::Str|Inner::BStr|Inner::CStr,b'\n'|b'\r'|CharKind::SUFFIXB,_)=> true,
      (Inner::Str|Inner::BStr|Inner::CStr,b'\\',Some(CharKind::SUFFIXB))=> {
        self.escape_suffix=true;
        false
      },
      (Inner::Char|Inner::BChar,b'\\',Some(CharKind::SUFFIXB))=> {
        self.escape_suffix=true;
        false
      }
      _=> false,
    };

    if cond {
      self.len-=self.inner.suffix_len();
    }

    cond.then_some(TokenHint::new(self.len,hint))
  }
*/













