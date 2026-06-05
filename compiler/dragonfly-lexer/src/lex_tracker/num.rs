
use crate::{
  prelude::*,
  literal::*,
};


pub struct NumLexTracker {
  len: usize,
  inner: Inner,
}

enum Inner {
  Dec(DecProps),
  Hex,
  Oct,
  Bin,
}

#[repr(transparent)]
#[derive(Debug,PartialEq,Eq,Default)]
struct DecProps {
  flags: u8,
}


impl NumLexTracker {
  #[inline(always)]
  fn new(inner: Inner)-> Self {
    let len=inner.prefix_len()
    .map(NonZeroUsize::get)
    .unwrap_or(1);

    Self {
      len,
      inner,
    }
  }

  pub fn try_start(buf: &[u8])-> Option<Self> {
    let inner=match buf {
      buf if buf.starts_with(int::HEX_PREFIX)=> Inner::Hex,
      buf if buf.starts_with(int::OCT_PREFIX)=> Inner::Oct,
      buf if buf.starts_with(int::BIN_PREFIX)=> Inner::Bin,
      buf if is_ascii_dec_start(buf[0])=> Inner::Dec(DecProps::new()),
      _=> return None
    };

    Some(Self::new(inner))
  }

  // TODO(nate): just fix me..
  // hex,bin,oct prefixes dont work properly
  // and also need to support them for floats
  pub fn try_finish(&mut self,buf: &[u8])-> Option<TokenHint> {
    self.len+=1;
    match (&mut self.inner,buf[0]) {
      (Inner::Hex,b'0'..=b'9'|b'a'..=b'f'|b'A'..=b'F'|b'_')=> return None,
      (Inner::Oct,b'0'..=b'7'|b'_')=> return None,
      (Inner::Bin,b'0'|b'1'|b'_')=> return None,
      (Inner::Dec(_),b'0'..=b'9'|b'_')=> return None,
      (Inner::Dec(props),b'.')=> {
        let hint=TokenHint::new(self.len,TokenHintKind::INFERRED_FLOAT);
        return props.toggle_dot().then_some(hint);
      },
      (Inner::Dec(props),b'e'|b'E')=> {
        let hint=TokenHint::new(self.len,TokenHintKind::INFERRED_FLOAT);
        return props.toggle_exp().then_some(hint);
      },
      (Inner::Dec(props),b'-')=> {
        let hint=TokenHint::new(self.len,TokenHintKind::INFERRED_FLOAT);
        return props.toggle_neg().then_some(hint);
      },
      _=> self.len-=1,
    };

    for &suffix in int::INT_SUFFIXES {
      if buf.starts_with_ignore_ascii_case(suffix) {
        self.len+=suffix.len();
        let kind=IntKind::from_suffix(suffix);
        return Some(TokenHint::new(self.len,TokenHintKind::Int(kind)));
      }
    }

    match &self.inner {
      Inner::Dec(_)=> (),
      _=> return Some(TokenHint::new(self.len,TokenHintKind::INFERRED_INT)),
    }

    for &suffix in float::FLOAT_SUFFIXES {
      if buf.starts_with_ignore_ascii_case(suffix) {
        self.len+=suffix.len();
        let kind=FloatKind::from_suffix(suffix);

        return Some(TokenHint::new(self.len,TokenHintKind::Float(kind)));
      }
    }

    match &self.inner {
      Inner::Dec(props) if props.none()=> Some(TokenHint::new(self.len,TokenHintKind::INFERRED_INT)),
      Inner::Dec(_)=> Some(TokenHint::new(self.len,TokenHintKind::INFERRED_FLOAT)),
      _=> unreachable!()
    }
  }

  #[inline(always)]
  pub const fn prefix_len(&self)-> Option<NonZeroUsize> {
    self.inner.prefix_len()
  }
}

impl Inner {
  #[inline(always)]
  pub const fn prefix_len(&self)-> Option<NonZeroUsize> {
    let len=match self {
      Inner::Bin=> int::BIN_PREFIX.len(),
      Inner::Oct=> int::OCT_PREFIX.len(),
      Inner::Dec(_)=> return None,
      Inner::Hex=> int::HEX_PREFIX.len(),
    };

    NonZeroUsize::new(len)
  }}

impl DecProps {
  const DOT: u8=1;
  const EXP: u8=2;
  const NEG: u8=3;


  #[inline]
  const fn new()-> Self {
    Self { flags: 0 }
  }

  const fn dot(&self)-> bool {
    self.flags & (1 << Self::DOT) != 0
  }

  const fn toggle_dot(&mut self)-> bool {
    let hit=self.dot();
    self.flags^=1 << Self::DOT;
    hit
  }

  const fn exp(&self)-> bool {
    self.flags & (1 << Self::EXP) != 0
  }

  const fn toggle_exp(&mut self)-> bool {
    let hit=self.exp();
    self.flags^=1 << Self::EXP;
    hit
  }

  const fn neg(&self)-> bool {
    self.flags & (1 << Self::NEG) != 0
  }

  const fn toggle_neg(&mut self)-> bool {
    let hit=self.neg();
    self.flags^=1 << Self::NEG;
    hit
  }

  const fn none(&self)-> bool {
    self.flags==0
  }
}



#[inline(always)]
const fn is_ascii_dec_start(ch: u8)-> bool {
  match ch {
    ch if ch.is_ascii_digit()=> true,
    b'-'=> true,
    _=> false,
  }
}



