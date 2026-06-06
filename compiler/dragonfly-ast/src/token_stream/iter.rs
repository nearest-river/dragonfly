
use crate::prelude::*;


pub struct Iter<'a> {
  idx: usize,
  stream: &'a TokenStream,
}

impl<'a> Iter<'a> {
  #[inline(always)]
  pub(crate) fn new(stream: &'a TokenStream)-> Self {
    Self {
      idx: 0,
      stream,
    }
  }
}

impl<'a> Iterator for Iter<'a> {
  type Item=&'a TokenTree;

  #[inline(always)]
  fn next(&mut self)-> Option<Self::Item> {
    if self.idx==self.stream.len() {
      return None;
    }

    let tt=&self.stream.0[self.idx];
    self.idx+=1;
    Some(tt)
  }
}

impl<'a> IntoIterator for &'a TokenStream {
  type Item=&'a TokenTree;
  type IntoIter=Iter<'a>;

  fn into_iter(self)-> Self::IntoIter {
    self.iter()
  }
}


impl FromIterator<TokenTree> for TokenStream {
  fn from_iter<T: IntoIterator<Item=TokenTree>>(tokens: T)-> Self {
    TokenStream::new(Vec::from_iter(tokens))
  }
}


impl FromIterator<TokenStream> for TokenStream {
  fn from_iter<I: IntoIterator<Item = TokenStream>>(streams: I) -> Self {
    let mut token_stream=TokenStream::new(Vec::new());

    for stream in streams {
      token_stream.push_stream(stream);
    }

    token_stream
  }
}




