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
  source_map: SourceMap,
}

static SESSION_GLOBALS: SessionGlobals=SessionGlobals {
  source_map: SourceMap::new()
};

#[inline(always)]
pub const fn source_map<'a>()-> &'a SourceMap {
  &SESSION_GLOBALS.source_map
}

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












