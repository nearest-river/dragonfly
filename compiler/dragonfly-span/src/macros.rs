
#[macro_export]
macro_rules! max {
  ($x:expr,$y:expr)=> {
    if $x > $y {
      $x
    } else {
      $y
    }
  };
}


#[macro_export]
macro_rules! min {
  ($x:expr,$y:expr)=> {
    if $x < $y {
      $x
    } else {
      $y
    }
  };
}



