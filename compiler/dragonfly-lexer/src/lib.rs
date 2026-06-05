
mod util;
mod lexer;
mod token_hint;
mod lex_tracker;

pub mod span;
pub mod error;
pub mod ident;
pub mod token;
pub mod literal;
pub mod prelude;
pub mod comment;
pub mod lifetime;

pub use crate::{
  token::*,
  span::Span,
  lexer::Lexer,
};


// Make sure that the Unicode version of the dependencies is the same.
const _: () = {
  let ident=unicode_ident::UNICODE_VERSION;
  let properties=unicode_properties::UNICODE_VERSION;


  if properties.0 != ident.0 as u64
    || properties.1 != ident.1 as u64
    || properties.2 != ident.2 as u64
  {
    panic!(
      "unicode-properties and unicode-ident must use the same Unicode version, \
      `unicode_properties::UNICODE_VERSION` and `unicode_ident::UNICODE_VERSION` are \
      different."
    );
  }
};



