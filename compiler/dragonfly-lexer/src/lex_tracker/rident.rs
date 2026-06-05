
use crate::{
  ident,
  prelude::*,
};


pub struct RIdentLexTracker {
  len: usize,
}

impl RIdentLexTracker {
  #[inline(always)]
  const fn new()-> Self {
    Self { len: 1 }
  }

  pub fn try_start(buf: &[u8])-> Option<Self> {
    if !buf.starts_with(ident::RAW_PREFIX) {
      return None;
    }

    if buf.len()>ident::RAW_PREFIX_LEN && let Err(_)=is_continue(&buf[ident::RAW_PREFIX_LEN..]) {
      return None;
    }

    Some(Self::new())
  }

  #[inline(always)]
  pub fn try_finish(&mut self,buf: &[u8])-> Option<TokenHint> {
    self.len+=1;
    let hint=is_continue(buf).err()?;
    Some(TokenHint::new(self.len,hint))
  }

  pub const fn prefix_len(&self)-> usize {
    ident::RAW_PREFIX_LEN
  }
}

fn is_continue(buf: &[u8])-> Result<(),TokenHintKind> {
  let s=match str::from_utf8(buf) {
    Ok(s)=> s,
    Err(err) if 0==err.valid_up_to()=> return Ok(()),
    Err(err)=> {
      let reason=Reason::ParseIdentErr(ParseIdentErr::from_utf8_err(err));
      return Err(TokenHintKind::Illegal(reason));
    },
  };

  let ch0=match s.chars().next() {
    Some(ch)=> ch,
    None=> {
      let reason=Reason::ParseIdentErr(ParseIdentErr::new(0));
      return Err(TokenHintKind::Illegal(reason));
    },
  };

  if unicode_ident::is_xid_continue(ch0) {
    return Ok(());
  } else {
    return Err(TokenHintKind::RIdent);
  }
}









