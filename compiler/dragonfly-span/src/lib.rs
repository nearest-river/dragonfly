#![allow(internal_features)]
#![feature(step_trait,rustc_attrs)]

#[macro_use]
mod macros;

mod span;
mod ident;
mod symbol;

pub use span::Span;
pub use ident::Ident;
pub use symbol::{
  sym,
  Symbol,
};


pub struct SessionGlobals {

}


















