
use std::hash::Hash;


pub const trait Indexable: Sized+Clone+Copy+PartialEq+Eq+PartialOrd+Ord+Hash {
  fn as_idx(self)-> usize;
}


impl const Indexable for u8 {
  #[inline(always)]
  fn as_idx(self)-> usize {
    self as usize
  }
}

#[cfg(any(target_pointer_width="16",target_pointer_width="32",target_pointer_width="64"))]
impl const Indexable for u16 {
  #[inline(always)]
  fn as_idx(self)-> usize {
    self as usize
  }
}

#[cfg(any(target_pointer_width="32",target_pointer_width="64"))]
impl const Indexable for u32 {
  #[inline(always)]
  fn as_idx(self)-> usize {
    self as usize
  }
}

#[cfg(any(target_pointer_width="64"))]
impl const Indexable for u64 {
  #[inline(always)]
  fn as_idx(self)-> usize {
    self as usize
  }
}

impl const Indexable for usize {
  #[inline(always)]
  fn as_idx(self)-> usize {
    self as usize
  }
}





