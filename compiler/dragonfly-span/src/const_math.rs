

#[inline(always)]
pub const fn max(x: u32,y: u32)-> u32 {
  if x>y {
    x
  } else {
    y
  }
}

#[inline(always)]
pub const fn min(x: u32,y: u32)-> u32 {
  if x<y {
    x
  } else {
    y
  }
}



