
use crate::prelude::*;
use lexical::{
  ParseFloatOptions,
  NumberFormatBuilder,
};


pub static FLOAT_SUFFIXES: &[&[u8]]=&[
  FloatKind::F16.suffix(),
  FloatKind::F32.suffix(),
  FloatKind::F64.suffix(),
  FloatKind::F128.suffix(),
];

pub const DIGIT_SEP: u8=b'_';
pub const FMT: u128=NumberFormatBuilder::new()
  .digit_separator(NonZeroU8::new(DIGIT_SEP))
  .leading_digit_separator(true)
  .internal_digit_separator(true)
  .trailing_digit_separator(true)
  .build_strict();


#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub enum FloatKind {
  F16,
  F32,
  #[default]
  F64,
  F128,
}


impl FloatKind {
  #[inline]
  pub const fn as_suffix_str(&self)-> &'static str {
    match self {
      Self::F16=> "f16",
      Self::F32=> "f32",
      Self::F64=> "f64",
      Self::F128=> "f128",
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &'static [u8] {
    self.as_suffix_str().as_bytes()
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }

  #[inline]
  pub const fn from_suffix(suffix: &[u8])-> Option<Self> {
    let kind=match suffix {
      b"f16"|b"F16"=> Self::F16,
      b"f32"|b"F32"=> Self::F32,
      b"f64"|b"F64"=> Self::F64,
      b"f128"|b"F128"=> Self::F128,
      _=> return None,
    };

    Some(kind)
  }

  #[inline]
  pub fn try_parse(self,buf: &[u8])-> Result<TokenKind,ParseNumberErr> {
    let options=ParseFloatOptions::new();

    let repr=self.repr_without_ty_suffix(buf);
    match self {
      Self::F16=> unimplemented!("f16 is not stable yet."),
      Self::F32=> { lexical::parse_with_options::<f32,_,FMT>(repr,&options)?; },
      Self::F64=> { lexical::parse_with_options::<f64,_,FMT>(repr,&options)?; },
      Self::F128=> unimplemented!("f128 is not stable yet."),
    };

    Ok(TokenKind::Literal(LiteralKind::Float(self)))
  }

  #[inline]
  fn repr_without_ty_suffix<'a>(&self,buf: &'a [u8])-> &'a [u8] {
    if buf.ends_with_ignore_ascii_case(self.suffix()) {
      let end=buf.len()-self.suffix_len();
      return &buf[..end];
    }

    buf
  }
}



#[inline]
pub fn parse(buf: &[u8],kind: Option<FloatKind>)-> TokenKind {
  let token_kind=match kind {
    None=> guess_float_kind(buf),
    Some(kind)=> kind.try_parse(buf),
  };

  match token_kind {
    Ok(token_kind)=> token_kind,
    Err(err)=> TokenKind::Illegal(Reason::ParseNumberErr(err)),
  }
}

static PRIORITY_QUEUE: &[FloatKind]=&[
  FloatKind::F64,
  FloatKind::F128,
];

fn guess_float_kind(buf: &[u8])-> Result<TokenKind,ParseNumberErr> {
  for kind in PRIORITY_QUEUE {
    match kind.try_parse(buf) {
      Ok(kind)=> return Ok(kind),
      Err(ParseNumberErr::Overflow(_))=> continue,
      Err(err)=> return Err(err),
    }
  }

  // FIXME(nate): idk this 0 is a placeholder val.
  Err(ParseNumberErr::Overflow(0))
}



