
use crate::prelude::*;


pub trait Parse: Sized {
  fn parse(input: ParseBuffer)-> Result<Self>;
}

pub struct ParseBuffer;






