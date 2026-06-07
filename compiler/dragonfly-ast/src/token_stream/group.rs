
use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Formatter,
};


#[derive(Clone)]
pub struct Group {
  pub(crate) span: Span,
  pub(crate) stream: TokenStream,
  pub(crate) delimiter: Delimiter,
}

#[derive(Clone,PartialEq,Eq,Debug,Copy)]
pub enum Delimiter {
  Paren,
  Brace,
  Bracket,
  Invisible, // Reserved for future
}

impl Group {
  #[inline]
  pub(crate) fn new(delimiter: Delimiter,stream: TokenStream)-> Self {
    let span=Self::calc_span(stream.as_ref())
    .unwrap_or(Span::call_site());

    Self {
      span,
      stream,
      delimiter,
    }
  }

  #[inline(always)]
  pub(crate) fn new_empty(delimiter: Delimiter,span: Span)-> Self {
    Self {
      span,
      delimiter,
      stream: TokenStream::new_empty(),
    }
  }

  #[inline(always)]
  fn calc_span(tts: TokenStreamRef<'_>)-> Option<Span> {
    let start=tts.first()?.span();
    let end=tts.last()?.span();

    Some(start.join(end))
  }

  #[inline(always)]
  pub const fn delimiter(&self)-> Delimiter {
    self.delimiter
  }

  #[inline(always)]
  pub const fn outer_span(&self)-> Span {
    self.span
  }

  #[inline(always)]
  pub const fn span(&self)-> Span {
    self.span
  }

  #[inline(always)]
  pub const fn span_open(&self)-> Span {
    self.span.first_byte()
  }

  #[inline(always)]
  pub const fn span_close(&self)-> Span {
    self.span.last_byte()
  }

  #[inline(always)]
  pub fn into_stream(self)-> TokenStream {
    self.stream
  }

  #[inline(always)]
  pub fn inner_stream(&self)-> TokenStreamRef<'_> {
    if self.delimiter==Delimiter::Invisible {
      unimplemented!();
    }

    assert!(self.stream.len()>=2);
    let start=1usize;
    let end=self.stream.len()-1;

    let view=&self.stream.0[start..end];
    TokenStreamRef::new(view)
  }
}

impl Eq for Group {}
impl PartialEq for Group {
  fn eq(&self,other: &Self)-> bool {
    self.delimiter==other.delimiter && self.stream==other.stream
  }
}

impl ToTokenStream for Group {
  #[inline(always)]
  fn into_token_stream(self)-> TokenStream {
    TokenTree::Group(self)
    .into_token_stream()
  }
}

impl Delimiter {
  pub const PAREN_CHARS: (char,char)=('(',')');
  pub const BRACE_CHARS: (char,char)=('{','}');
  pub const BRACKET_CHARS: (char,char)=('[',']');

  pub const PAREN_BYTES: (u8,u8)=(b'(',b')');
  pub const BRACE_BYTES: (u8,u8)=(b'{',b'}');
  pub const BRACKET_BYTES: (u8,u8)=(b'[',b']');

  #[inline(always)]
  pub const fn as_chars(&self)-> (char,char) {
    match self {
      Self::Paren=> Self::PAREN_CHARS,
      Self::Brace=> Self::BRACE_CHARS,
      Self::Bracket=> Self::BRACKET_CHARS,
      Self::Invisible=> unimplemented!(),
    }
  }

  #[inline(always)]
  pub const fn as_bytes(&self)-> (u8,u8) {
    match self {
      Self::Paren=> Self::PAREN_BYTES,
      Self::Brace=> Self::BRACE_BYTES,
      Self::Bracket=> Self::BRACKET_BYTES,
      Self::Invisible=> unimplemented!(),
    }
  }

  #[inline(always)]
  pub const fn opening_char(&self)-> char {
    self.as_chars().0
  }

  #[inline(always)]
  pub const fn opening_byte(&self)-> u8 {
    self.as_bytes().0
  }

  #[inline(always)]
  pub const fn closing_char(&self)-> char {
    self.as_chars().1
  }

  #[inline(always)]
  pub const fn closing_byte(&self)-> u8 {
    self.as_bytes().1
  }
}


impl Debug for Group {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    if !f.alternate() {
      let mut dbg=f.debug_struct(stringify!(Group));

      dbg.field("delimiter",&self.delimiter);
      dbg.field("stream",&self.stream);
      dbg.field("span",&self.span);

      return dbg.finish();
    }

    // We attempt to match libproc_macro's formatting.
    // Empty parens: ()
    // Nonempty parens: (...)
    // Empty brackets: []
    // Nonempty brackets: [...]
    // Empty braces: { }
    // Nonempty braces: { ... }
    match self.delimiter {
      Delimiter::Paren=> {
        if self.inner_stream().is_empty() {
          return f.write_str("()");
        }

        let mut dbg=f.debug_tuple("");
        for entry in self.inner_stream() {
          dbg.field(entry);
        }

        dbg.finish()
      },
      Delimiter::Brace=> {
        f.debug_set()
        .entries(self.inner_stream())
        .finish()
      }
      Delimiter::Bracket=> {
        f.debug_list()
        .entries(self.inner_stream())
        .finish()
      }
      Delimiter::Invisible=> Debug::fmt(&self.stream,f)
    }
  }
}



