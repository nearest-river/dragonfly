
use crate::prelude::*;
use std::{
  ops::Deref,
  marker::PhantomData,
  fmt::{
    self,
    Debug,
    Formatter,
  },
};




#[repr(transparent)]
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct TokenStreamRef<'a> {
  pub(crate) inner: &'a [TokenTree],
}

#[derive(Clone)]
pub struct TokenStreamRefIter<'a> {
  stream: TokenStreamRef<'a>,
  idx: usize,
}

pub struct SplitToken<'a> {
  sep: Token,
  start: usize,
  cursor: usize,
  stream: TokenStreamRef<'a>,
}

pub struct Split<'a,S: AsRef<[TokenTree]>> {
  pat: S,
  start: usize,
  cursor: usize,
  stream: TokenStreamRef<'a>
}

pub struct SplitAnyToken<'a,P: AsRef<[Token]>> {
  pats: P,
  start: usize,
  cursor: usize,
  stream: TokenStreamRef<'a>,
}

pub struct SplitAny<'a,S: AsRef<[TokenTree]>,P: AsRef<[S]>> {
  pats: P,
  start: usize,
  cursor: usize,
  stream: TokenStreamRef<'a>,
  _marker: PhantomData<S>,
}


impl<'a> TokenStreamRef<'a> {
  #[inline(always)]
  pub fn new(inner: &'a [TokenTree])-> Self {
    Self {
      inner
    }
  }

  #[inline]
  fn total_span(&self)-> Option<Span> {
    let start=self.inner.first()?.span();
    let end=self.inner.last()?.span();

    Some(start.join(end))
  }

  #[inline]
  pub fn span(&self)-> Span {
    self.total_span()
    .unwrap_or_default()
  }

  pub fn linear_search(&self,tt: &TokenTree)-> Option<usize> {
    for i in 0..self.len() {
      if tt==&self.inner[i] {
        return Some(i);
      }
    }

    return None;
  }

  pub fn linear_search_by<F: std::ops::Fn(&TokenTree)-> bool>(&self,predicate: F)-> Option<usize> {
    for i in 0..self.len() {
      if predicate(&self.inner[i]) {
        return Some(i);
      }
    }

    return None;
  }

  #[inline(always)]
  pub fn split_token(&self,sep: Token)-> SplitToken<'a> {
    SplitToken::new(*self,sep)
  }

  #[inline(always)]
  pub fn split<S: AsRef<[TokenTree]>>(&self,pat: S)-> Split<'a,S> {
    Split::new(*self,pat)
  }

  #[inline(always)]
  pub fn split_any_token<P: AsRef<[Token]>>(&self,pats: P)-> SplitAnyToken<'a,P> {
    SplitAnyToken::new(*self,pats)
  }

  #[inline(always)]
  pub fn split_any<S: AsRef<[TokenTree]>,P: AsRef<[S]>>(&self,pats: P)-> SplitAny<'a,S,P> {
    SplitAny::new(*self,pats)
  }

  #[inline(always)]
  pub fn take(&self,count: usize)-> TokenStreamRef<'a> {
    TokenStreamRef::new(&self.inner[..count])
  }

  #[inline(always)]
  pub fn take_while<F: FnMut(&TokenTree) -> bool>(&self,mut predicate: F)-> TokenStreamRef<'a> {
    let count=self.iter()
    .take_while(|tt| predicate(*tt))
    .count();

    TokenStreamRef::new(&self.inner[..count])
  }
}

impl<'a> AsRef<[TokenTree]> for TokenStreamRef<'a> {
  #[inline(always)]
  fn as_ref(&self)-> &'a [TokenTree] {
    self.inner
  }
}

impl<'a> Deref for TokenStreamRef<'a> {
  type Target=[TokenTree];
  #[inline(always)]
  fn deref(&self)-> &'a Self::Target {
    self.inner
  }
}

impl<'a> IntoIterator for TokenStreamRef<'a> {
  type Item=&'a TokenTree;
  type IntoIter=TokenStreamRefIter<'a>;
  fn into_iter(self)-> Self::IntoIter {
    TokenStreamRefIter {
      idx: 0,
      stream: self,
    }
  }
}

impl<'a> Iterator for TokenStreamRefIter<'a> {
  type Item=&'a TokenTree;
  fn next(&mut self)-> Option<Self::Item> {
    if self.idx==self.stream.len() {
      return None;
    }

    let tt=&self.stream.inner[self.idx];
    self.idx+=1;
    Some(tt)
  }
}

impl Debug for TokenStreamRef<'_> {
  #[allow(clippy::borrow_deref_ref)]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    f.write_str("TokenStreamRef ")?;
    f.debug_list()
    .entries(&*self.inner)
    .finish()
  }
}


impl ToTokenStream for TokenStreamRef<'_> {
  #[inline]
  fn into_token_stream(self)-> TokenStream {
    self.iter()
    .map(Clone::clone)
    .collect()
  }
}


impl<'a> SplitToken<'a> {
  #[inline(always)]
  pub(crate) fn new(stream: TokenStreamRef<'a>,sep: Token)-> Self {
    Self {
      sep,
      stream,
      start: 0,
      cursor: 0,
    }
  }
}

impl<'a,S: AsRef<[TokenTree]>> Split<'a,S> {
  #[inline(always)]
  pub(crate) fn new(stream: TokenStreamRef<'a>,pat: S)-> Self {
    Self {
      pat,
      stream,
      start: 0,
      cursor: 0,
    }
  }
}

impl<'a,S: AsRef<[TokenTree]>,P: AsRef<[S]>> SplitAny<'a,S,P> {
  #[inline(always)]
  pub(crate) fn new(stream: TokenStreamRef<'a>,pats: P)-> Self {
    Self {
      pats,
      stream,
      start: 0,
      cursor: 0,
      _marker: PhantomData,
    }
  }

  #[inline]
  fn matches_any_pat(&self)-> Option<&[TokenTree]> {
    for pat in self.pats.as_ref() {
      let pat=pat.as_ref();
      let slice=&self.stream.inner[self.cursor..];
      if slice.len()<pat.len() {
        continue;
      }

      let mut iter=pat.iter()
      .zip(slice);
      if iter.all(|(tt0,tt1)| tt0==tt1) {
        return Some(pat);
      }
    }

    None
  }
}

impl<'a,P: AsRef<[Token]>> SplitAnyToken<'a,P> {
  #[inline(always)]
  pub(crate) fn new(stream: TokenStreamRef<'a>,pats: P)-> Self {
    Self {
      pats,
      stream,
      start: 0,
      cursor: 0,
    }
  }

  #[inline]
  fn matches_any(&self,token: &Token)-> bool {
    self.pats.as_ref()
    .iter()
    .any(|pat| pat==token)
  }
}

impl<'a,P: AsRef<[Token]>> Iterator for SplitAnyToken<'a,P> {
  type Item=TokenStreamRef<'a>;
  fn next(&mut self)-> Option<Self::Item> {
    for tt in &self.stream[self.cursor..] {
      match tt {
        TokenTree::Token(token) if self.matches_any(token)=> {
          let stream=&self.stream.inner[self.start..self.cursor];
          self.start=self.cursor;
          self.cursor+=1;
          return Some(TokenStreamRef::new(stream));
        },
        _=> self.cursor+=1,
      }
    }

    if self.start==self.cursor {
      return None;
    }

    let stream=&self.stream.inner[self.start..self.cursor];
    self.start=self.cursor;
    Some(TokenStreamRef::new(stream))
  }
}


impl<'a> Iterator for SplitToken<'a> {
  type Item=TokenStreamRef<'a>;
  fn next(&mut self)-> Option<Self::Item> {
    for tt in &self.stream[self.cursor..] {
      match tt {
        TokenTree::Token(token) if &self.sep==token=> {
          let stream=&self.stream.inner[self.start..self.cursor];
          self.start=self.cursor;
          self.cursor+=1;
          return Some(TokenStreamRef::new(stream));
        },
        _=> self.cursor+=1,
      }
    }

    if self.start==self.cursor {
      return None;
    }

    let stream=&self.stream.inner[self.start..self.cursor];
    self.start=self.cursor;
    Some(TokenStreamRef::new(stream))
  }
}

impl<'a,S: AsRef<[TokenTree]>,P: AsRef<[S]>> Iterator for SplitAny<'a,S,P> {
  type Item=TokenStreamRef<'a>;
  fn next(&mut self)-> Option<Self::Item> {
    while self.cursor<self.stream.len() {
      let pat=match self.matches_any_pat() {
        Some(pat)=> pat,
        None=> {
          self.cursor+=1;
          continue;
        },
      };

      let stream=&self.stream.inner[self.start..self.cursor];
      // since `pat` is borrowed, let's lazily assign start=cursor
      let start=self.cursor;

      self.cursor+=pat.as_ref().len();
      self.start=start;
      return Some(TokenStreamRef::new(stream));
    }

    if self.start==self.cursor {
      return None;
    }

    let stream=&self.stream.inner[self.start..self.cursor];
    self.start=self.cursor;
    Some(TokenStreamRef::new(stream))
  }
}


impl<'a,S: AsRef<[TokenTree]>> Iterator for Split<'a,S> {
  type Item=TokenStreamRef<'a>;
  fn next(&mut self)-> Option<Self::Item> {
    while self.cursor<self.stream.len() {
      if !self.pat_matches() {
        self.cursor+=1;
        continue;
      }

      let stream=&self.stream.inner[self.start..self.cursor];
      self.start=self.cursor;
      self.cursor+=self.pat.as_ref().len();
      return Some(TokenStreamRef::new(stream));
    }

    if self.start==self.cursor {
      return None;
    }

    let stream=&self.stream.inner[self.start..self.cursor];
    self.start=self.cursor;
    Some(TokenStreamRef::new(stream))
  }
}


impl<'a,S: AsRef<[TokenTree]>> Split<'a,S> {
  fn pat_matches(&self)-> bool {
    let pat=self.pat.as_ref();
    let slice=&self.stream.inner[self.cursor..];
    if slice.len()<pat.len() {
      return false;
    }

    pat.iter()
    .zip(slice)
    .all(|(tt0,tt1)| tt0==tt1)
  }
}





