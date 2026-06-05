
use crate::prelude::*;
use std::fmt::{
  self,
  Debug,
  Formatter,
};


#[derive(Clone,PartialEq,Eq)]
pub enum TokenKind {
  // wtf?
  Ident,
  RawIdent,
  RawLifetime,
  Illegal(Reason),
  Comment(CommentKind),
  Lifetime {
    starts_with_number: bool,
  },

  // literals
  Literal(LiteralKind),

  // weird stuff
  // Eof,
  Unsafe,
  Super,
  Crate,

  // Punctuations
  At,
  And,
  AndAnd,
  AndEq,
  Caret,
  CaretEq,
  Colon,
  Comma,
  Dollar,
  Dot,
  DotDot,
  DotDotDot,
  DotDotEq,
  Equal,
  EqualEqual,
  FatArrow,
  Ge,
  Gt,
  Le,
  Lt,
  LArrow,
  Minus,
  MinusEq,
  NotEq,
  Not,
  Or,
  OrEq,
  OrOr,
  PathSep,
  Percent,
  PercentEq,
  Plus,
  PlusEq,
  Pound,
  Question,
  RArrow,
  SemiColon,
  Shl,
  Shr,
  ShlEq,
  ShrEq,
  Slash,
  SlashEq,
  Star,
  StarEq,

  Underscore,

  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,

  // keywords
  As,
  In,
  Fn,
  Struct,
  Const,
  Let,
  Static,
  Enum,
  Impl,
  Trait,
  Auto,
  Async,
  Type,
  Extern,
  Mod,
  Use,
  Default,
  Dyn,
  Ref,
  Pub,

  // Control Flow,
  Break,
  Continue,
  Return,
  Yeet,
  Await,

  If,
  Else,
  Match,
  While,
  For,
  Loop,
  Macro,
  Move,
  Mut,
  Raw,
  SelfType,
  SelfValue,
  Typeof,
  Union,
  Where,
  Yield,
}

impl Debug for TokenKind {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    match self {
      // wtf?
      Self::Ident=> f.write_str(stringify!(Ident))?,
      Self::RawIdent=> f.write_str(stringify!(RawIdent))?,
      Self::RawLifetime=> f.write_str(stringify!(RawLifetime))?,
      Self::Illegal(reason)=> Debug::fmt(&reason,f)?,
      Self::Comment(kind)=> Debug::fmt(&kind,f)?,
      Self::Lifetime { .. }=> f.write_str(stringify!(Lifetime))?,

      // literals
      Self::Literal(kind)=> Debug::fmt(&kind,f)?,

      // weird stuff
      Self::Unsafe=> f.write_str(stringify!(Unsafe))?,
      Self::Super=> f.write_str(stringify!(Super))?,
      Self::Crate=> f.write_str(stringify!(Crate))?,

      // Punctuations
      Self::At=> f.write_str(stringify!(At))?,
      Self::And=> f.write_str(stringify!(And))?,
      Self::AndAnd=> f.write_str(stringify!(AndAnd))?,
      Self::AndEq=> f.write_str(stringify!(AndEq))?,
      Self::Caret=> f.write_str(stringify!(Caret))?,
      Self::CaretEq=> f.write_str(stringify!(CaretEq))?,
      Self::Colon=> f.write_str(stringify!(Colon))?,
      Self::Comma=> f.write_str(stringify!(Comma))?,
      Self::Dollar=> f.write_str(stringify!(Dollar))?,
      Self::Dot=> f.write_str(stringify!(Dot))?,
      Self::DotDot=> f.write_str(stringify!(DotDot))?,
      Self::DotDotDot=> f.write_str(stringify!(DotDotDot))?,
      Self::DotDotEq=> f.write_str(stringify!(DotDotEq))?,
      Self::Equal=> f.write_str(stringify!(Equal))?,
      Self::EqualEqual=> f.write_str(stringify!(EqualEqual))?,
      Self::FatArrow=> f.write_str(stringify!(FatArrow))?,
      Self::Ge=> f.write_str(stringify!(Ge))?,
      Self::Gt=> f.write_str(stringify!(Gt))?,
      Self::Le=> f.write_str(stringify!(Le))?,
      Self::Lt=> f.write_str(stringify!(Lt))?,
      Self::LArrow=> f.write_str(stringify!(LArrow))?,
      Self::Minus=> f.write_str(stringify!(Minus))?,
      Self::MinusEq=> f.write_str(stringify!(MinusEq))?,
      Self::NotEq=> f.write_str(stringify!(NotEq))?,
      Self::Not=> f.write_str(stringify!(Not))?,
      Self::Or=> f.write_str(stringify!(Or))?,
      Self::OrEq=> f.write_str(stringify!(OrEq))?,
      Self::OrOr=> f.write_str(stringify!(OrOr))?,
      Self::PathSep=> f.write_str(stringify!(PathSep))?,
      Self::Percent=> f.write_str(stringify!(Percent))?,
      Self::PercentEq=> f.write_str(stringify!(PercentEq))?,
      Self::Plus=> f.write_str(stringify!(Plus))?,
      Self::PlusEq=> f.write_str(stringify!(PlusEq))?,
      Self::Pound=> f.write_str(stringify!(Pound))?,
      Self::Question=> f.write_str(stringify!(Question))?,
      Self::RArrow=> f.write_str(stringify!(RArrow))?,
      Self::SemiColon=> f.write_str(stringify!(SemiColon))?,
      Self::Shl=> f.write_str(stringify!(Shl))?,
      Self::Shr=> f.write_str(stringify!(Shr))?,
      Self::ShlEq=> f.write_str(stringify!(ShlEq))?,
      Self::ShrEq=> f.write_str(stringify!(ShrEq))?,
      Self::Slash=> f.write_str(stringify!(Slash))?,
      Self::SlashEq=> f.write_str(stringify!(SlashEq))?,
      Self::Star=> f.write_str(stringify!(Star))?,
      Self::StarEq=> f.write_str(stringify!(StarEq))?,

      Self::Underscore=> f.write_str(stringify!(Underscore))?,

      Self::LParen=> f.write_str(stringify!(LParen))?,
      Self::RParen=> f.write_str(stringify!(RParen))?,
      Self::LBrace=> f.write_str(stringify!(LBrace))?,
      Self::RBrace=> f.write_str(stringify!(RBrace))?,
      Self::LBracket=> f.write_str(stringify!(LBracket))?,
      Self::RBracket=> f.write_str(stringify!(RBracket))?,

      // keywords
      Self::As=> f.write_str(stringify!(As))?,
      Self::In=> f.write_str(stringify!(In))?,
      Self::Fn=> f.write_str(stringify!(Fn))?,
      Self::Struct=> f.write_str(stringify!(Struct))?,
      Self::Const=> f.write_str(stringify!(Const))?,
      Self::Let=> f.write_str(stringify!(Let))?,
      Self::Static=> f.write_str(stringify!(Static))?,
      Self::Enum=> f.write_str(stringify!(Enum))?,
      Self::Impl=> f.write_str(stringify!(Impl))?,
      Self::Trait=> f.write_str(stringify!(Trait))?,
      Self::Auto=> f.write_str(stringify!(Auto))?,
      Self::Async=> f.write_str(stringify!(Async))?,
      Self::Type=> f.write_str(stringify!(Type))?,
      Self::Extern=> f.write_str(stringify!(Extern))?,
      Self::Mod=> f.write_str(stringify!(Mod))?,
      Self::Use=> f.write_str(stringify!(Use))?,
      Self::Default=> f.write_str(stringify!(Default))?,
      Self::Dyn=> f.write_str(stringify!(Dyn))?,
      Self::Ref=> f.write_str(stringify!(Ref))?,
      Self::Pub=> f.write_str(stringify!(Pub))?,

      // Control Flow
      Self::Break=> f.write_str(stringify!(Break))?,
      Self::Continue=> f.write_str(stringify!(Continue))?,
      Self::Return=> f.write_str(stringify!(Return))?,
      Self::Yeet=> f.write_str(stringify!(Yeet))?,
      Self::Await=> f.write_str(stringify!(Await))?,

      Self::If=> f.write_str(stringify!(If))?,
      Self::Else=> f.write_str(stringify!(Else))?,
      Self::Match=> f.write_str(stringify!(Match))?,
      Self::While=> f.write_str(stringify!(While))?,
      Self::For=> f.write_str(stringify!(For))?,
      Self::Loop=> f.write_str(stringify!(Loop))?,
      Self::Macro=> f.write_str(stringify!(Macro))?,
      Self::Move=> f.write_str(stringify!(Move))?,
      Self::Mut=> f.write_str(stringify!(Mut))?,
      Self::Raw=> f.write_str(stringify!(Raw))?,
      Self::SelfType=> f.write_str(stringify!(SelfType))?,
      Self::SelfValue=> f.write_str(stringify!(SelfValue))?,
      Self::Typeof=> f.write_str(stringify!(Typeof))?,
      Self::Union=> f.write_str(stringify!(Union))?,
      Self::Where=> f.write_str(stringify!(Where))?,
      Self::Yield=> f.write_str(stringify!(Yield))?,
    }

    Ok(())
  }
}








