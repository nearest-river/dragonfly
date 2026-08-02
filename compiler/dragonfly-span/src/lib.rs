#![allow(internal_features)]
#![feature(rustc_attrs)]
#![feature(step_trait,const_trait_impl,structural_match,const_index,const_convert,const_cmp)]

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

use std::sync::RwLock;


pub struct SessionGlobals {
  pub source_map: RwLock<SourceMap>,
}

static SESSION_GLOBALS: SessionGlobals=SessionGlobals {
  source_map: RwLock::new(SourceMap::new())
};


#[inline(always)]
pub fn with_source<T: Sized>(source_id: SourceId,f: impl FnOnce(&Source)-> T)-> T {
  let source_map=SESSION_GLOBALS.source_map
  .read()
  .expect("failed to read source map");

  f(&source_map[source_id])
}

#[inline(always)]
pub fn with_source_mut<T: Sized>(source_id: SourceId,f: impl FnOnce(&mut Source)-> T)-> T {
  let mut source_map=SESSION_GLOBALS.source_map
  .write()
  .expect("failed to write source map");

  f(&mut source_map[source_id])
}












