
use crate::{
  ident,
  prelude::*,
};


pub static PREFIX: &[u8]=b"'";
pub static RAW_PREFIX: &[u8]=b"'r#";

pub static SUFFIX: &[u8]=b"";
pub static RAW_SUFFIX: &[u8]=b"";



pub fn parse(buf: &[u8],raw: bool)-> TokenKind {
  let repr_buf=if raw {
    if !buf.starts_with(RAW_PREFIX) || buf.len()<=RAW_PREFIX.len() {
      let reason=Reason::ParseIdentErr(ParseIdentErr::new(0));
      return TokenKind::Illegal(reason);
    }

    &buf[RAW_PREFIX.len()..]
  } else {
    &buf[PREFIX.len()..]
  };

  let repr=str::from_utf8(repr_buf)
  .expect("aint it supposed to be utf-8 eh?");

  match raw {
    false if let Err(off)=ident::validate_ident(repr)=> TokenKind::Illegal(Reason::ParseIdentErr(ParseIdentErr::new(off))),
    true if let Err(off)=ident::validate_ident_raw(repr)=> TokenKind::Illegal(Reason::ParseIdentErr(ParseIdentErr::new(off))),
    true=> TokenKind::RawLifetime,
    false=> TokenKind::Lifetime,
  }
}



