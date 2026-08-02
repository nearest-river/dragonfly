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
  source_map: RwLock<SourceMap>,
}

static SESSION_GLOBALS: SessionGlobals=SessionGlobals {
  source_map: RwLock::new(SourceMap::new())
};


impl SessionGlobals {
  #[inline(always)]
  pub fn with<T>(&self,f: impl FnOnce(&SessionGlobals)-> T)-> T {
    f(&self)
  }
}


#[inline(always)]
pub fn with_session_globals<T>(f: impl FnOnce(&SessionGlobals)-> T)-> T {
  SESSION_GLOBALS.with(f)
}

#[inline(always)]
pub fn with_source_map<T: Sized>(f: impl FnOnce(&SourceMap)-> T)-> T {
  let source_map=SESSION_GLOBALS.source_map
  .read()
  .expect("failed to read source map");

  f(&source_map)
}

#[inline(always)]
pub fn with_source_map_mut<T: Sized>(f: impl FnOnce(&mut SourceMap)-> T)-> T {
  let mut source_map=SESSION_GLOBALS.source_map
  .write()
  .expect("failed to write source map");

  f(&mut source_map)
}












