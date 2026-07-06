
use std::mem;
use crate::prelude::*;


pub struct CommentLexTracker {
  len: usize,
  inner: Inner,
}

enum Inner {
  Line,
  DocLine,
  Block(BStack),
  DocBlock(BStack),
}

impl CommentLexTracker {
  fn new(inner: Inner)-> Self {
    let len=inner.prefix_len();
    Self {
      len,
      inner,
    }
  }

  pub fn try_start(buf: &[u8])-> Option<Self> {
    let kind=CommentKind::parse(buf)?;

    let this=match kind {
      CommentKind::Line=> Self::new(Inner::Line),
      CommentKind::DocLine=> Self::new(Inner::DocLine),
      CommentKind::Block=> Self::new(Inner::Block(BStack::new())),
      CommentKind::DocBlock=> Self::new(Inner::DocBlock(BStack::new())),
    };

    Some(this)
  }

  pub fn try_finish(&mut self,buf: &[u8])-> Option<TokenHint> {
    self.len+=1;

    let comment_kind=self.kind();
    let stack=match (&mut self.inner,buf[0]) {
      (Inner::Block(stack)|Inner::DocBlock(stack),_)=> stack,
      (Inner::Line|Inner::DocLine,b'\n')=> return Some(TokenHint::new(self.len,self.hint_kind())),
      (Inner::Line|Inner::DocLine,_)=> return None,
    };

    if buf.starts_with(comment_kind.prefix()) {
      assert!(stack.push());
      return None;
    }

    if buf.starts_with(comment_kind.suffix()) {
      stack.pop()?;
    }

    if !stack.is_empty() {
      return None;
    }

    self.len+=comment_kind.suffix_len();
    // we had already added 1 to `self.len` once which belongs to the span of `suffix_len`
    self.len-=1;
    match &self.inner {
      Inner::DocLine|Inner::Line=> unreachable!("line comments are handled seperately"),
      Inner::Block(_)=> Some(TokenHint::new(self.len,self.hint_kind())),
      Inner::DocBlock(_)=> Some(TokenHint::new(self.len,self.hint_kind())),
    }
  }

  #[inline(always)]
  const fn hint_kind(&self)-> TokenHintKind {
    TokenHintKind::Comment(self.kind())
  }

  #[inline(always)]
  const fn kind(&self)-> CommentKind {
    self.inner.kind()
  }

  #[inline(always)]
  pub const fn prefix_len(&self)-> usize {
    self.inner.prefix_len()
  }
}

impl Inner {
  #[inline(always)]
  const fn kind(&self)-> CommentKind {
    match self {
      Inner::Line=> CommentKind::Line,
      Inner::DocLine=> CommentKind::DocLine,
      Inner::Block(_)=> CommentKind::Block,
      Inner::DocBlock(_)=> CommentKind::DocBlock,
    }
  }

  #[inline(always)]
  const fn prefix_len(&self)-> usize {
    self.kind().prefix_len()
  }
}


#[repr(transparent)]
pub(crate) struct BStack(u128);

impl BStack {
  #[inline(always)]
  const fn new()-> Self {
    Self(0x1)
  }

  #[inline(always)]
  const fn len(&self)-> u32 {
    self.0.trailing_ones()
  }

  #[inline(always)]
  const fn capacity(&self)-> u32 {
    8*mem::size_of::<Self>() as u32
  }

  #[inline(always)]
  const fn is_empty(&self)-> bool {
    self.0==0
  }

  #[inline(always)]
  const fn push(&mut self)-> bool {
    if self.len()>=self.capacity() {
      return false;
    }

    self.0<<=1;
    self.0&=1;
    true
  }

  #[inline(always)]
  fn pop(&mut self)-> Option<()> {
    if self.is_empty() {
      return None;
    }

    self.0>>=1;
    Some(())
  }
}



















