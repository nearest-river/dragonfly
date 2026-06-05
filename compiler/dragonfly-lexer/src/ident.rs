
use crate::prelude::*;


pub const RAW_PREFIX: &[u8]=b"r#";
pub const RAW_PREFIX_LEN: usize=RAW_PREFIX.len();

pub fn parse(buf: &[u8],raw: bool)-> TokenKind {
  let repr_buf=if raw {
    if !buf.starts_with(RAW_PREFIX) || buf.len()<=RAW_PREFIX.len() {
      let reason=Reason::ParseIdentErr(ParseIdentErr::new(0));
      return TokenKind::Illegal(reason);
    }

    &buf[RAW_PREFIX.len()..]
  } else {
    buf
  };

  let repr=str::from_utf8(repr_buf)
  .expect("aint it supposed to be utf-8 eh?");

  match raw {
    false if let Err(off)=validate_ident(repr)=> TokenKind::Illegal(Reason::ParseIdentErr(ParseIdentErr::new(off))),
    true if let Err(off)=validate_ident_raw(repr)=> TokenKind::Illegal(Reason::ParseIdentErr(ParseIdentErr::new(off))),
    true=> TokenKind::RawIdent,
    false=> TokenKind::Ident,
  }
}

#[inline]
#[track_caller]
pub(crate) fn validate_ident(repr: &str)-> Result<(),usize> {
  debug_assert!(!repr.is_empty());

  if repr.bytes().all(|byte| byte.is_ascii_digit()) {
    return Err(0);
  }

  fn ident_ok(repr: &str)-> Result<(),usize> {
    let mut chars=repr.chars()
    .enumerate();

    let (idx,ch0)=chars.next()
    .unwrap();

    if !is_start(ch0) {
      return Err(idx);
    }

    for (i,ch) in chars {
      if !is_continue(ch) {
        return Err(i);
      }
    }

    Ok(())
  }

  ident_ok(repr)
}

#[inline]
#[track_caller]
pub(crate) fn validate_ident_raw(repr: &str)-> Result<(),usize> {
  match repr {
    "_"|"super"|"self"|"Self"|"crate"=> Err(0),
    repr=> validate_ident(repr)
  }
}

#[inline(always)]
pub(crate) fn is_start(ch: char)-> bool {
  ch=='_' || unicode_ident::is_xid_start(ch)
}

#[inline(always)]
pub(crate) fn is_continue(ch: char)-> bool {
  unicode_ident::is_xid_continue(ch)
}






