
use crate::prelude::*;
use lexical::{
  NumberFormatBuilder,
  ParseIntegerOptions,
};


pub static HEX_PREFIX: &[u8]=b"0x";
pub static OCT_PREFIX: &[u8]=b"0o";
pub static DEC_PREFIX: &[u8]=b"";
pub static BIN_PREFIX: &[u8]=b"0b";

pub const DIGIT_SEP: u8=b'_';

pub const HEX_BASE_PREFIX: u8=b'x';
pub const DEC_BASE_PREFIX: u8=0;
pub const OCT_BASE_PREFIX: u8=b'o';
pub const BIN_BASE_PREFIX: u8=b'b';

pub const HEX_RADIX: u8=16;
pub const DEC_RADIX: u8=10;
pub const OCT_RADIX: u8=8;
pub const BIN_RADIX: u8=2;

pub const HEX_FMT: u128=number_fmt_from_prefix(HEX_PREFIX);
pub const DEC_FMT: u128=number_fmt_from_prefix(DEC_PREFIX);
pub const OCT_FMT: u128=number_fmt_from_prefix(OCT_PREFIX);
pub const BIN_FMT: u128=number_fmt_from_prefix(BIN_PREFIX);



pub static INT_SUFFIXES: &[&[u8]]=&[
  IntKind::U8.suffix(),
  IntKind::U16.suffix(),
  IntKind::U32.suffix(),
  IntKind::U64.suffix(),
  IntKind::U128.suffix(),
  IntKind::U256.suffix(),
  IntKind::Usize.suffix(),
  IntKind::I8.suffix(),
  IntKind::I16.suffix(),
  IntKind::I32.suffix(),
  IntKind::I64.suffix(),
  IntKind::I128.suffix(),
  IntKind::I256.suffix(),
  IntKind::Isize.suffix(),
];

#[allow(dead_code,clippy::unnecessary_cast)]
// definitely gonne be used later.
mod consts {
  pub(in super) const I8_MAX: u128=i8::MAX.cast_unsigned() as u128;
  pub(in super) const I8_MIN: u128=i8::MIN.cast_unsigned() as u128;
  pub(in super) const I16_MAX: u128=i16::MAX.cast_unsigned() as u128;
  pub(in super) const I16_MIN: u128=i16::MIN.cast_unsigned() as u128;
  pub(in super) const I32_MAX: u128=i32::MAX.cast_unsigned() as u128;
  pub(in super) const I32_MIN: u128=i32::MIN.cast_unsigned() as u128;
  pub(in super) const I64_MAX: u128=i64::MAX.cast_unsigned() as u128;
  pub(in super) const I64_MIN: u128=i64::MIN.cast_unsigned() as u128;
  pub(in super) const I128_MAX: u128=i128::MAX.cast_unsigned() as u128;
  pub(in super) const I128_MIN: u128=i128::MIN.cast_unsigned() as u128;
  pub(in super) const ISIZE_MAX: u128=isize::MAX.cast_unsigned() as u128;
  pub(in super) const ISIZE_MIN: u128=isize::MIN.cast_unsigned() as u128;

  pub(in super) const U8_MAX: u128=u8::MAX as u128;
  pub(in super) const U8_MIN: u128=u8::MIN as u128;
  pub(in super) const U16_MAX: u128=u16::MAX as u128;
  pub(in super) const U16_MIN: u128=u16::MIN as u128;
  pub(in super) const U32_MAX: u128=u32::MAX as u128;
  pub(in super) const U32_MIN: u128=u32::MIN as u128;
  pub(in super) const U64_MAX: u128=u64::MAX as u128;
  pub(in super) const U64_MIN: u128=u64::MIN as u128;
  pub(in super) const U128_MAX: u128=u128::MAX as u128;
  pub(in super) const U128_MIN: u128=u128::MIN as u128;
  pub(in super) const USIZE_MAX: u128=usize::MAX as u128;
  pub(in super) const USIZE_MIN: u128=usize::MIN as u128;
}


#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Default)]
pub enum IntKind {
  U8,
  U16,
  U32,
  U64,
  U128,
  U256,
  Usize,
  I8,
  I16,
  #[default]
  I32,
  I64,
  I128,
  I256,
  Isize,
}




impl IntKind {
  #[inline]
  pub const fn as_suffix_str(&self)-> &'static str {
    match self {
      Self::U8=> "u8",
      Self::U16=> "u16",
      Self::U32=> "u32",
      Self::U64=> "u64",
      Self::U128=> "u128",
      Self::U256=> "u256",
      Self::Usize=> "usize",
      Self::I8=> "i8",
      Self::I16=> "i16",
      Self::I32=> "i32",
      Self::I64=> "i64",
      Self::I128=> "i128",
      Self::I256=> "i256",
      Self::Isize=> "isize",
    }
  }

  #[inline(always)]
  pub const fn suffix(&self)-> &[u8] {
    self.as_suffix_str().as_bytes()
  }

  #[inline]
  pub const fn suffix_len(&self)-> usize {
    self.suffix().len()
  }

  #[inline]
  pub const fn from_suffix(suffix: &[u8])-> Option<Self> {
    let kind=match suffix {
      b"u8"|b"U8"=> Self::U8,
      b"u16"|b"U16"=> Self::U16,
      b"u32"|b"U32"=> Self::U32,
      b"u64"|b"U64"=> Self::U64,
      b"u128"|b"U128"=> Self::U128,
      b"u256"|b"U256"=> Self::U256,
      b"usize"=> Self::Usize,
      b"i8"|b"I8"=> Self::I8,
      b"i16"|b"I16"=> Self::I16,
      b"i32"|b"I32"=> Self::I32,
      b"i64"|b"I64"=> Self::I64,
      b"i128"|b"I128"=> Self::I128,
      b"i256"|b"I256"=> Self::I256,
      b"isize"=> Self::Isize,
      suffix if suffix.eq_ignore_ascii_case(b"usize")=> Self::Usize,
      suffix if suffix.eq_ignore_ascii_case(b"isize")=> Self::Isize,
      _=> return None,
    };

    Some(kind)
  }

  #[inline]
  pub const fn signed(&self)-> bool {
    match self {
      Self::I8|Self::I16|Self::I32|Self::I64|Self::I128|Self::I256|Self::Isize=> true,
      Self::U8|Self::U16|Self::U32|Self::U64|Self::U128|Self::U256|Self::Usize=> false,
    }
  }

  #[inline]
  pub fn try_parse(self,buf: &[u8])-> Result<TokenKind,ParseNumberErr> {
    let repr=self.repr_without_ty_suffix(buf);

    match repr {
      repr if repr.starts_with(HEX_PREFIX)=> self._try_parse::<HEX_FMT>(repr),
      repr if repr.starts_with(OCT_PREFIX)=> self._try_parse::<OCT_FMT>(repr),
      repr if repr.starts_with(BIN_PREFIX)=> self._try_parse::<BIN_FMT>(repr),
      repr=> self._try_parse::<DEC_FMT>(repr)
    }
  }

  #[inline]
  fn _try_parse<const FMT: u128>(self,repr: &[u8])-> Result<TokenKind,ParseNumberErr> {
    let options=ParseIntegerOptions::new();

    match self {
      Self::U8=> { lexical::parse_with_options::<u8,_,FMT>(repr,&options)?; },
      Self::I8=> { lexical::parse_with_options::<i8,_,FMT>(repr,&options)?; },
      Self::U16=> { lexical::parse_with_options::<u16,_,FMT>(repr,&options)?; },
      Self::I16=> { lexical::parse_with_options::<i16,_,FMT>(repr,&options)?; },
      Self::U32=> { lexical::parse_with_options::<u32,_,FMT>(repr,&options)?; },
      Self::I32=> { lexical::parse_with_options::<i32,_,FMT>(repr,&options)?; },
      Self::U64=> { lexical::parse_with_options::<u64,_,FMT>(repr,&options)?; },
      Self::I64=> { lexical::parse_with_options::<i64,_,FMT>(repr,&options)?; },
      Self::U128=> { lexical::parse_with_options::<u128,_,FMT>(repr,&options)?; },
      Self::I128=> { lexical::parse_with_options::<i128,_,FMT>(repr,&options)?; },
      Self::Usize=> { lexical::parse_with_options::<usize,_,FMT>(repr,&options)?; },
      Self::Isize=> { lexical::parse_with_options::<isize,_,FMT>(repr,&options)?; },
      Self::U256=> unimplemented!("i256 is not yet implemented"),
      Self::I256=> unimplemented!("u256 is not yet implemented"),
    };

    Ok(TokenKind::Literal(LiteralKind::Int(self)))
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

static PRIORITY_QUEUE: &[IntKind]=&[
  IntKind::I32,
  IntKind::U32,
  IntKind::I64,
  IntKind::U64,
  IntKind::I256,
  IntKind::U128,
];

#[inline]
pub fn parse(buf: &[u8],kind: Option<IntKind>)-> TokenKind {
  let token_kind=match kind {
    None=> guess_int_kind(buf),
    Some(kind)=> {
      let hi=buf.len()-kind.suffix_len();
      // if the suffix was specified
      // then it must not be passed to the int/float parser.
      kind.try_parse(&buf[..hi])
    },
  };


  match token_kind {
    Ok(token_kind)=> token_kind,
    Err(err)=> TokenKind::Illegal(Reason::ParseNumberErr(err)),
  }
}

#[inline]
fn guess_int_kind(buf: &[u8])-> Result<TokenKind,ParseNumberErr> {
  for kind in PRIORITY_QUEUE {
    match kind.try_parse(buf) {
      Ok(token_kind)=> return Ok(token_kind),
      Err(ParseNumberErr::Overflow(_))=> continue,
      Err(err)=> return Err(err),
    }
  }

  // FIXME(nate): idk this 0 is a placeholder val.
  Err(ParseNumberErr::Overflow(0))
}


#[inline]
const fn number_fmt_from_prefix(prefix: &[u8])-> u128 {
  let fmt=NumberFormatBuilder::new()
  .digit_separator(NonZeroU8::new(DIGIT_SEP))
  .leading_digit_separator(true)
  .internal_digit_separator(true)
  .trailing_digit_separator(true);

  let (radix,base_prefix)=match prefix {
    b"0b"=> (BIN_RADIX,BIN_BASE_PREFIX),
    b"0o"=> (OCT_RADIX,OCT_BASE_PREFIX),
    b""=> (DEC_RADIX,DEC_BASE_PREFIX),
    b"0x"=> (HEX_RADIX,HEX_BASE_PREFIX),
    _=> return fmt.build_strict(),
  };


  fmt.radix(radix)
  .base_prefix(NonZeroU8::new(base_prefix))
  .build_strict()
}






