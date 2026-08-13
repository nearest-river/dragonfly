
use crate::*;


pub struct Ident {
  pub name: Symbol,
  pub span: Span,
}

impl Ident {
  #[inline(always)]
  pub fn new(name: Symbol,span: Span)-> Self {
    debug_assert!(!name.is_empty());
    Ident {
      name,
      span,
    }
  }

  #[inline(always)]
  pub fn with_dummy_span(name: Symbol)-> Self {
    Ident::new(name,Span::dummy())
  }

  pub fn as_str(&self)-> &str {
    self.name.as_str()
  }
}






