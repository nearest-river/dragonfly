#![allow(internal_features)]
#![feature(step_trait,rustc_attrs,const_cmp,const_trait_impl,structural_match)]

#[macro_use]
mod macros;

mod span;
mod ident;
mod symbol;
mod source_map;

pub use span::Span;
pub use ident::Ident;
pub use source_map::*;
pub use symbol::{
  sym,
  Symbol,
};



pub struct SessionGlobals {
  pub source_map: SourceMap,
}


















