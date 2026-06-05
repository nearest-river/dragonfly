
pub mod int;
pub mod float;
pub mod string;
pub mod boolean;
pub mod character;

use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Formatter,
};


#[derive(Clone,PartialEq,Eq)]
pub enum LiteralKind {
  Float(FloatKind),
  Int(IntKind),
  Str(StrKind),
  Char(CharKind),
  Bool,
}


impl Debug for LiteralKind {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    match self {
      Self::Float(kind)=> Debug::fmt(&kind,f)?,
      Self::Int(kind)=> Debug::fmt(&kind,f)?,
      Self::Str(kind)=> Debug::fmt(&kind,f)?,
      Self::Char(kind)=> Debug::fmt(&kind,f)?,
      Self::Bool=> f.write_str(stringify!(Bool))?,
    }

    Ok(())
  }
}






