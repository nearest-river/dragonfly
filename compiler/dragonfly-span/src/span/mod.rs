

use std::{
  ops::Range,
  cmp::Ordering,
  fmt::{
    self,
    Debug,
    Display,
    Formatter,
  },
};




#[derive(Default,Clone,Copy,PartialEq,Eq,Hash)]
pub struct Span {
  pub lo: u32,
  pub hi: u32,
}


impl Span {
  #[inline(always)]
  pub const fn new(lo: u32,hi: u32)-> Self {
    Self { lo, hi }
  }

  #[inline(always)]
  pub const fn call_site()-> Self {
    Self::new(0,0)
  }

  #[inline(always)]
  pub const fn byte_range(&self)-> Range<usize> {
    let (lo,hi)=(self.lo as usize,self.hi as usize);

    lo..hi
  }

  #[inline(always)]
  pub const fn join(&self,other: Span)-> Span {
    Span {
      lo: min!(self.lo,other.lo),
      hi: max!(self.hi,other.hi),
    }
  }

  #[inline(always)]
  pub const fn first_byte(&self)-> Span {
    let Span { lo, hi }=*self;

    Span {
      lo,
      hi: min!(lo.saturating_add(1),hi),
    }
  }

  #[inline(always)]
  pub const fn last_byte(&self)-> Span {
    let Span { lo, hi }=*self;

    Span {
      lo: max!(hi.saturating_sub(1),lo),
      hi,
    }
  }

  #[inline(always)]
  pub const fn is_call_site(&self)-> bool {
    self.lo==0 && self.hi==0
  }
}

impl Ord for Span {
  #[inline(always)]
  fn cmp(&self,other: &Self)-> Ordering {
    if self==other {
      Ordering::Equal
    } else if self.hi<other.lo {
      Ordering::Less
    } else if self.lo>other.hi {
      Ordering::Greater
    } else {
      let lo_ord=self.lo.cmp(&other.lo);
      let hi_ord=self.hi.cmp(&other.hi);
      lo_ord.then(hi_ord)
    }
  }
}

impl PartialOrd for Span {
  #[inline(always)]
  fn partial_cmp(&self,other: &Self)-> Option<Ordering> {
    Some(Ord::cmp(self,other))
  }
}

impl Debug for Span {
  #[inline(always)]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    let Span { lo, hi }=*self;
    if f.alternate() {
      write!(f,"[{lo}..{hi}]")
    } else {
      write!(f,"[{lo},{hi})")
    }
  }
}

impl Display for Span {
  #[inline(always)]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    write!(f,"{:#?}",self)
  }
}






