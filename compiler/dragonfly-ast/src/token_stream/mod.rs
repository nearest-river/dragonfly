
mod view;
mod iter;
mod parse;
pub mod error;
pub mod group;

use crate::prelude::*;
use dragonfly_lexer::Lexer;

use std::{
  sync::Arc,
  fmt::{
    self,
    Debug,
    Formatter,
  },
};


pub use view::*;
pub use group::Group;




#[derive(Clone,PartialEq,Eq)]
pub struct TokenStream(pub(crate) Arc<Vec<TokenTree>>);

#[derive(Clone,PartialEq,Eq)]
pub enum TokenTree {
  Token(Token),
  Group(Group),
}

pub trait ToTokenStream {
  fn into_token_stream(self)-> TokenStream;

  #[inline]
  fn to_token_stream(&self)-> TokenStream
  where 
    Self: Clone
  {
    self.clone()
    .into_token_stream()
  }
}


impl TokenStream {
  #[inline(always)]
  pub fn new(tts: Vec<TokenTree>)-> Self {
    Self(Arc::new(tts))
  }

  #[inline(always)]
  pub fn new_empty()-> Self {
    Self::new(Vec::new())
  }

  pub fn parse(buf: &[u8])-> Result<TokenStream,Error> {
    parse::parse(Lexer::new(buf))
  }

  pub fn is_empty(&self)-> bool {
    self.0.is_empty()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn get(&self, index: usize) -> Option<&TokenTree> {
    self.0.get(index)
  }

  pub fn iter(&self)-> iter::Iter<'_> {
    iter::Iter::new(self)
  }

  pub fn push(&mut self,tt: TokenTree) {
    let vec_mut=Arc::make_mut(&mut self.0);
    vec_mut.push(tt);
  }

  pub fn push_stream(&mut self,stream: TokenStream) {
    let vec_mut=Arc::make_mut(&mut self.0);
    vec_mut.extend(stream.iter().cloned());
  }

  #[inline(always)]
  pub fn as_ref(&self)-> TokenStreamRef<'_> {
    TokenStreamRef::new(&self.0)
  }

  #[inline(always)]
  pub fn view(&self,start: usize)-> TokenStreamRef<'_> {
    TokenStreamRef::new(&self.0[start..])
  }

  #[inline]
  fn total_span(&self)-> Option<Span> {
    let start=self.0.first()?.span();
    let end=self.0.last()?.span();

    Some(start.join(end))
  }

  #[inline]
  pub fn span(&self)-> Span {
    self.total_span()
    .unwrap_or_default()
  }

  #[inline(always)]
  pub fn linear_search(&self,tt: &TokenTree)-> Option<usize> {
    self.as_ref().linear_search(tt)
  }

  #[inline(always)]
  pub fn linear_search_by<F: std::ops::Fn(&TokenTree)-> bool>(&self,predicate: F)-> Option<usize> {
    self.as_ref().linear_search_by(predicate)
  }
}


impl Default for TokenStream {
  #[inline(always)]
  fn default()-> Self {
    TokenStream::new(Vec::new())
  }
}

impl TokenTree {
  #[inline(always)]
  pub fn span(&self)-> Span {
    match self {
      Self::Token(token)=> token.span,
      Self::Group(group)=> group.span,
    }
  }
}

impl From<Group> for TokenTree {
  #[inline(always)]
  fn from(group: Group)-> Self {
    TokenTree::Group(group)
  }
}

impl From<Token> for TokenTree {
  #[inline(always)]
  fn from(token: Token)-> Self {
    TokenTree::Token(token)
  }
}

impl PartialEq<Token> for TokenTree {
  #[inline(always)]
  fn eq(&self,other: &Token)-> bool {
    match self {
      Self::Token(token)=> token==other,
      Self::Group(_)=> false,
    }
  }
}

impl PartialEq<Group> for TokenTree {
  #[inline]
  fn eq(&self,other: &Group)-> bool {
    match self {
      Self::Group(group)=> group==other,
      Self::Token(_)=> false,
    }
  }
}

impl PartialEq<TokenStream> for TokenTree {
  #[inline]
  fn eq(&self,other: &TokenStream)-> bool {
    if other.len()!=1 {
      return false;
    }

    let other=other.0
    .iter()
    .next()
    .unwrap();

    self==other
  }
}

impl ToTokenStream for TokenTree {
  #[inline]
  fn into_token_stream(self)-> TokenStream {
    TokenStream::new(vec![self])
  }
}

impl ToTokenStream for TokenStream {
  #[inline(always)]
  fn into_token_stream(self)-> TokenStream {
    self
  }
}

impl Debug for TokenStream {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    f.write_str("TokenStream ")?;
    f.debug_list()
    .entries(self.as_ref())
    .finish()
  }
}


impl Debug for TokenTree {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if f.alternate() {
      return match self {
        Self::Token(leaf)=> Debug::fmt(leaf,f),
        Self::Group(group)=> Debug::fmt(group,f)
      };
    }

    match self {
      Self::Token(leaf)=> {
        f.debug_tuple(stringify!(Token))
        .field(leaf)
        .finish()
      },
      Self::Group(group)=> {
        f.debug_tuple(stringify!(Group))
        .field(group)
        .finish()
      },
    }
  }
}




