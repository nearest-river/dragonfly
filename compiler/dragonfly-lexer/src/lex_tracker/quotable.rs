

use crate::{
  ident,
  prelude::*,
};


pub struct QuotableTracker {
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
  CharOrLifetime,
  Lifetime,
  RawLifetime,
}

impl QuotableTracker {
  pub fn try_start(buf: &[u8])-> Option<Self> {
    let inner=match buf {
      buf if buf.starts_with(StrKind::Str.prefix())=> Inner::Str,
      buf if buf.starts_with(StrKind::BStr.prefix())=> Inner::BStr,
      buf if buf.starts_with(StrKind::CStr.prefix())=> Inner::CStr,
      buf if buf.starts_with(StrKind::RStr.prefix())=> Inner::RStr,
      buf if buf.starts_with(StrKind::BRStr.prefix())=> Inner::BRStr,
      buf if buf.starts_with(StrKind::CRStr.prefix())=> Inner::CRStr,
      buf if buf.starts_with(CharKind::BChar.prefix())=> Inner::BChar,
      buf if buf.starts_with(lifetime::RAW_PREFIX)=> Inner::RawLifetime,
      buf if buf.starts_with(CharKind::Char.prefix())=> {
        if CharKind::Char.prefix()==lifetime::PREFIX {
          Inner::CharOrLifetime
        } else {
          Inner::Char
        }
      },
      buf if buf.starts_with(lifetime::PREFIX)=> Inner::Lifetime,
      _=> return None,
    };

    let len=inner.prefix_len();

    Some(Self {
      len,
      inner,
    })
  }

  pub fn try_finish(&mut self,buf: &[u8])-> Option<TokenHint> {
    // used eq_ignore_ascii_case() cause its a const-fn..
    // both prefixes are in lowercase or dont care about case at all.
    assert!(lifetime::PREFIX.eq_ignore_ascii_case(CharKind::Char.prefix()));

    match self.inner {
      Inner::CharOrLifetime=> (),
      Inner::Lifetime=> unreachable!("lifetimes are handled immediately"),
      Inner::RawLifetime=> {
        let repr=str::from_utf8(buf)
        .inspect_err(|_| self.len+=1)
        .ok()?;
        // everything in the ascii world.
        // `None` should be an unrechable case.
        let ch0=repr.chars().next()?;

        if !ident::is_continue(ch0) {
          return Some(self.hint());
        }

        self.len+=1;
        return None;
      },
      _=> {
        if self.quote_terminated_token_ends(buf) {
          self.len+=self.suffix_len();
          return Some(self.hint());
        }

        self.len+=1;
        return None;
      },
    };

    if buf.starts_with(CharKind::Char.suffix()) {
      self.inner=Inner::Char;
      self.len+=self.suffix_len();
      return Some(self.hint());
    }

    let repr=str::from_utf8(buf)
    .inspect_err(|_| {
      // if it isnt ascii then it must be a utf-8 char,
      // and plain lifetimes must be ascii-identifiers.
      self.inner=Inner::Char;
      self.len+=1;
    })
    .ok()?;

    let ch0=repr.chars().next()?;
    if self.len==self.prefix_len() && !ident::is_start(ch0) {
      self.inner=Inner::Char;
    } else if !ident::is_continue(ch0) {
      self.inner=Inner::Lifetime;
      return Some(self.hint());
    }

    self.len+=1;
    None
  }

  #[inline(always)]
  fn quote_terminated_token_ends(&self,buf: &[u8])-> bool {
    assert!(!matches!(self.inner,Inner::CharOrLifetime|Inner::Lifetime|Inner::RawLifetime));
    buf.starts_with(self.inner.suffix())
  }

  #[inline(always)]
  const fn hint(&self)-> TokenHint {
    assert!(!matches!(self.inner,Inner::CharOrLifetime));
    let hint_kind=match self.inner {
      Inner::Str=> TokenHintKind::Str(StrKind::Str),
      Inner::BStr=> TokenHintKind::Str(StrKind::BStr),
      Inner::CStr=> TokenHintKind::Str(StrKind::CStr),
      Inner::RStr=> TokenHintKind::Str(StrKind::RStr),
      Inner::BRStr=> TokenHintKind::Str(StrKind::BRStr),
      Inner::CRStr=> TokenHintKind::Str(StrKind::CRStr),
      Inner::Char=> TokenHintKind::Char(CharKind::Char),
      Inner::BChar=> TokenHintKind::Char(CharKind::BChar),
      Inner::CharOrLifetime=> unreachable!(),
      Inner::Lifetime=> TokenHintKind::Lifetime,
      Inner::RawLifetime=> TokenHintKind::RawLifetime,
    };

    TokenHint::new(self.len,hint_kind)
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
  /// Panics if called on `Inner::CharOrLifetime`
  const fn suffix(&self)-> &[u8] {
    assert!(!matches!(self,Self::CharOrLifetime));

    match self {
      Self::Str=> StrKind::Str.suffix(),
      Self::BStr=> StrKind::BStr.suffix(),
      Self::CStr=> StrKind::CStr.suffix(),
      Self::RStr=> StrKind::RStr.suffix(),
      Self::BRStr=> StrKind::BRStr.suffix(),
      Self::CRStr=> StrKind::BRStr.suffix(),
      Self::Char=> CharKind::Char.suffix(),
      Self::BChar=> CharKind::BChar.suffix(),
      Self::Lifetime=> lifetime::SUFFIX,
      Self::RawLifetime=> lifetime::RAW_SUFFIX,
      Self::CharOrLifetime=> unreachable!()
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
      Self::Lifetime=> lifetime::PREFIX,
      Self::RawLifetime=> lifetime::RAW_PREFIX,
      Self::CharOrLifetime=> {
        // used eq_ignore_ascii_case() cause its a const-fn..
        // both prefixes are in lowercase or dont care about case at all.
        assert!(CharKind::Char.prefix().eq_ignore_ascii_case(lifetime::PREFIX));
        lifetime::PREFIX
      },
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






