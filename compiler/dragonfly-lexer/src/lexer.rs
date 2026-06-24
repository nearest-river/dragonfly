
use crate::*;
use prelude::*;
use literal::*;
use lex_tracker::*;



pub struct Lexer<'b> {
  // beginning of line.
  bol: usize,
  line: usize,
  cursor: usize,
  buf: &'b [u8],
}


impl<'b> Lexer<'b> {
  pub fn new(buf: &'b [u8])-> Self {
    Self {
      buf,
      bol: 0,
      line: 0,
      cursor: 0,
    }
  }

  #[inline(always)]
  const fn update(&mut self,count: usize) {
    self.cursor+=count;
  }

  #[inline(always)]
  pub const fn cursor(&self)-> usize {
    self.cursor
  }

  #[inline(always)]
  pub const fn col(&self)-> usize {
    self.cursor-self.bol
  }

  #[inline(always)]
  pub const fn line(&self)-> usize {
    self.line
  }

  #[inline(always)]
  pub const fn bol(&self)-> usize {
    self.bol
  }

  #[inline(always)]
  pub const fn is_eof(&self)-> bool {
    self.cursor>=self.buf.len()
  }

  #[inline(always)]
  pub const fn eof_in(&self,count: usize)-> bool {
    self.cursor+count>self.buf.len()
  }

  /*
  #[inline]
  pub fn parse(self)-> Result<TokenStream,LexErr> {
    parse::parse(self)
  }*/
}



impl<'a> Iterator for Lexer<'a> {
  type Item=Token;
  fn next(&mut self)-> Option<Token> {
    self.skip_whitespaces();
    if self.is_eof() {
      return None;
    }

    let TokenHint { len, hint }=self.next_sep();
    let end=self.cursor+len;
    let token=&self.buf[self.cursor..end];

    let span=Span {
      lo: self.cursor as u32,
      hi: end as u32,
    };

    self.update(len);

    let kind=match hint {
      TokenHintKind::Other=> Self::parse_lookup(token),
      TokenHintKind::RIdent=> ident::parse(token,true),
      TokenHintKind::Float(kind)=> float::parse(token,kind),
      TokenHintKind::Int(kind)=> int::parse(token,kind),
      TokenHintKind::Str(kind)=> string::parse(token,kind),
      TokenHintKind::Char(kind)=> character::parse(token,kind),
      TokenHintKind::Comment(kind)=> comment::parse(token,kind),
      TokenHintKind::Illegal(reason)=> TokenKind::Illegal(reason),
      TokenHintKind::Lifetime=> lifetime::parse(token,false),
      TokenHintKind::RawLifetime=> lifetime::parse(token,true),
    };

    Some(Token {
      span,
      kind,
    })
  }
}

impl Lexer<'_> {
  fn next_sep(&mut self)-> TokenHint {
    let ch0=self.buf[self.cursor];
    assert!(!ch0.is_ascii_whitespace());

    // Check singer char token
    if Self::is_special1(ch0) {
      return TokenHint::new(1,TokenHintKind::Other);
    }

    if !Self::is_special2plus(ch0) {
      let mut i=self.cursor;
      let mut tracker=Option::<LexTracker>::None;
      while i<self.buf.len() {
        let ch=self.buf[i];

        // handling numbers.
        if tracker.is_none() && let Some(num_tracker)=NumLexTracker::try_start(&self.buf[i..]) {
          let prefix_len=num_tracker.prefix_len()
          .map(NonZeroUsize::get);

          tracker=Some(LexTracker::Num(num_tracker));
          i+=prefix_len.unwrap_or(1);
          continue;
        } else if let Some(LexTracker::Num(num_tracker))=&mut tracker {
          if let Some(hint)=num_tracker.try_finish(&self.buf[i..]) {
            return hint;
          }
        }

        // handling string literals
        if tracker.is_none() && let Some(quotable_tracker)=QuotableTracker::try_start(&self.buf[i..]) {
          let prefix_len=quotable_tracker.prefix_len();
          tracker=Some(LexTracker::Quotable(quotable_tracker));

          i+=prefix_len;
          continue;
        } else if let Some(LexTracker::Quotable(quotable_tracker))=&mut tracker {
          if let Some(hint)=quotable_tracker.try_finish(&self.buf[i..]) {
            return hint;
          }
        }

        // rident handling
        if tracker.is_none() && let Some(rident_tracker)=RIdentLexTracker::try_start(&self.buf[i..]) {
          let prefix_len=rident_tracker.prefix_len();
          tracker=Some(LexTracker::RIdent(rident_tracker));

          i+=prefix_len;
          continue;
        } else if let Some(LexTracker::RIdent(rident_tracker))=&mut tracker {
          if let Some(hint)=rident_tracker.try_finish(&self.buf[i..]) {
            return hint;
          }
        }

        if tracker.is_none() {
          tracker=Some(LexTracker::Other);
        }

        // handling skips
        match &tracker {
          None|Some(LexTracker::Other)=> (),
          Some(_)=> {
            i+=1;
            continue;
          },
        }

        // breaks
        if ch.is_ascii_whitespace() {
          break;
        }

        if Self::is_special2plus(ch) && !ch.is_ascii_alphanumeric() {
          break;
        }

        if Self::is_special1(ch) {
          break;
        }

        i+=1;
      }

      return TokenHint::new(i-self.cursor,TokenHintKind::Other);
    }

    if let Some(mut com_tracker)=CommentLexTracker::try_start(&self.buf[self.cursor..]) {
      let mut i=self.cursor+com_tracker.prefix_len();
      while i<self.buf.len() {
        if let Some(hint)=com_tracker.try_finish(&self.buf[i..]) {
          return hint;
        }

        i+=1;
      }

      let reason=Reason::ParseCommentErr(ParseCommentErr::UnclosedDelimiter);
      return TokenHint::new(i-self.cursor,TokenHintKind::Illegal(reason));
    }

    let pats=match Self::seperator_pats(ch0) {
      Some(pats)=> pats,
      None=> unreachable!("`ch0` must have atleast one seperator pat, the other cases had been handles previously."),
    };

    // pattern img is defiened in ascending order of length.
    for &pat in pats.iter().rev() {
      let start=self.cursor;
      let end=self.cursor+pat.len();
      if !self.buf[start..end].starts_with(pat) {
        continue;
      }

      return TokenHint::new(pat.len(),TokenHintKind::Other);
    }

    TokenHint::new(self.buf.len()-self.cursor,TokenHintKind::Other)
  }
}


impl Lexer<'_> {
  #[inline(always)]
  fn parse_lookup(token: &[u8])-> TokenKind {
    match token {
      b"_"           => TokenKind::Underscore,
      b"("           => TokenKind::LParen,
      b")"           => TokenKind::RParen,
      b"{"           => TokenKind::LBrace,
      b"}"           => TokenKind::RBrace,
      b"["           => TokenKind::LBracket,
      b"]"           => TokenKind::RBracket,
      b"as"          => TokenKind::As,
      b"async"       => TokenKind::Async,
      b"await"       => TokenKind::Await,
      b"auto"        => TokenKind::Auto,
      b"break"       => TokenKind::Break,
      b"const"       => TokenKind::Const,
      b"continue"    => TokenKind::Continue,
      b"crate"       => TokenKind::Crate,
      b"default"     => TokenKind::Default,
      b"dyn"         => TokenKind::Dyn,
      b"else"        => TokenKind::Else,
      b"enum"        => TokenKind::Enum,
      b"extern"      => TokenKind::Extern,
      b"false"       => TokenKind::Literal(LiteralKind::Bool),
      b"fn"          => TokenKind::Fn,
      b"fly"         => TokenKind::Fly,
      b"for"         => TokenKind::For,
      b"goto"        => TokenKind::Goto,
      b"if"          => TokenKind::If,
      b"impl"        => TokenKind::Impl,
      b"in"          => TokenKind::In,
      b"let"         => TokenKind::Let,
      b"loop"        => TokenKind::Loop,
      b"macro"       => TokenKind::Macro,
      b"match"       => TokenKind::Match,
      b"mod"         => TokenKind::Mod,
      b"move"        => TokenKind::Move,
      b"mut"         => TokenKind::Mut,
      b"pub"         => TokenKind::Pub,
      b"raw"         => TokenKind::Raw,
      b"return"      => TokenKind::Return,
      b"Self"        => TokenKind::SelfType,
      b"self"        => TokenKind::SelfValue,
      b"static"      => TokenKind::Static,
      b"struct"      => TokenKind::Struct,
      b"super"       => TokenKind::Super,
      b"trait"       => TokenKind::Trait,
      b"true"        => TokenKind::Literal(LiteralKind::Bool),
      b"type"        => TokenKind::Type,
      b"typeof"      => TokenKind::Typeof,
      b"union"       => TokenKind::Union,
      b"unsafe"      => TokenKind::Unsafe,
      b"use"         => TokenKind::Use,
      b"where"       => TokenKind::Where,
      b"while"       => TokenKind::While,
      b"yield"       => TokenKind::Yield,
      b"yeet"        => TokenKind::Yeet,
      b"&"           => TokenKind::And,
      b"&&"          => TokenKind::AndAnd,
      b"&="          => TokenKind::AndEq,
      b"@"           => TokenKind::At,
      b"^"           => TokenKind::Caret,
      b"^="          => TokenKind::CaretEq,
      b":"           => TokenKind::Colon,
      b","           => TokenKind::Comma,
      b"$"           => TokenKind::Dollar,
      b"."           => TokenKind::Dot,
      b".."          => TokenKind::DotDot,
      b"..."         => TokenKind::DotDotDot,
      b"..="         => TokenKind::DotDotEq,
      b"="           => TokenKind::Equal,
      b"=="          => TokenKind::EqualEqual,
      b"=>"          => TokenKind::FatArrow,
      b">="          => TokenKind::Ge,
      b">"           => TokenKind::Gt,
      b"<-"          => TokenKind::LArrow,
      b"<="          => TokenKind::Le,
      b"<"           => TokenKind::Lt,
      b"-"           => TokenKind::Minus,
      b"-="          => TokenKind::MinusEq,
      b"!="          => TokenKind::NotEq,
      b"!"           => TokenKind::Not,
      b"|"           => TokenKind::Or,
      b"|="          => TokenKind::OrEq,
      b"||"          => TokenKind::OrOr,
      b"::"          => TokenKind::PathSep,
      b"%"           => TokenKind::Percent,
      b"%="          => TokenKind::PercentEq,
      b"+"           => TokenKind::Plus,
      b"+="          => TokenKind::PlusEq,
      b"#"           => TokenKind::Pound,
      b"?"           => TokenKind::Question,
      b"->"          => TokenKind::RArrow,
      b";"           => TokenKind::SemiColon,
      b"<<"          => TokenKind::Shl,
      b"<<="         => TokenKind::ShlEq,
      b">>"          => TokenKind::Shr,
      b">>="         => TokenKind::ShrEq,
      b"/"           => TokenKind::Slash,
      b"/="          => TokenKind::SlashEq,
      b"*"           => TokenKind::Star,
      b"*="          => TokenKind::StarEq,
      repr           => ident::parse(repr,false),
    }
  }


  #[inline]
  #[allow(clippy::match_like_matches_macro)]
  const fn is_special1(ch: u8)-> bool {
    match ch {
      b';'|b','|b'#'|b'?'=> true,
      b'('|b')'|b'{'|b'}'|b'['|b']'=> true,
      _=> false,
    }
  }

  #[inline]
  #[allow(clippy::match_like_matches_macro)]
  const fn is_special2plus(ch: u8)-> bool {
    match ch {
      b'&'|b'^'|b'>'|b'<'|b'|'|b'!'|b'='=> true,
      b':'|b'.'=> true,
      b'-'|b'%'|b'+'|b'/'|b'*'=> true,
      _=> false
    }
  }

  #[inline]
  const fn seperator_pats(ch: u8)-> Option<&'static [&'static [u8]]> {
    let seps: &'static [&'static [u8]]=match ch {
      b'&'=> &[b"&",b"&&",b"&="],
      b'^'=> &[b"^",b"^="],
      b':'=> &[b":",b"::"],
      b'.'=> &[b".",b"..",b"...",b"..="],
      b'='=> &[b"=",b"==",b"=>"],
      b'>'=> &[b">",b">=",b">>",b">>="],
      b'<'=> &[b"<",b"<=",b"<-",b"<<",b"<<="],
      b'-'=> &[b"-",b"-=",b"->"],
      b'!'=> &[b"!",b"!="],
      b'|'=> &[b"|",b"||",b"|="],
      b'%'=> &[b"%",b"%="],
      b'+'=> &[b"+",b"+="],
      b'/'=> &[b"/",b"/="],
      b'*'=> &[b"*",b"*="],
      _=> return None
    };

    Some(seps)
  }

  #[inline]
  const fn skip_whitespaces(&mut self) {
    let mut i=self.cursor;
    while i<self.buf.len() {
      match self.buf[i] {
        b'\n'|b'\r' if i+1<self.buf.len()=> self.bol=i+1,
        b'\n'|b'\r'|b'\t'|b'\x0C'|b' '=> (),
        _=> break,
      }

      i+=1;
    }

    self.update(i-self.cursor);
  }
}


