
use crate::prelude::*;
use dragonfly_lexer::{
  Lexer,
  Token,
  TokenKind,
};




type StackFrame=(Delimiter,Vec<TokenTree>);
pub(crate) fn parse(mut lexer: Lexer<'_>)-> Result<TokenStream,Error> {
  let mut vec=Vec::<TokenTree>::new();
  let mut stack=Vec::<StackFrame>::new();

  while let Some(token)=lexer.next() {
    if let Some(deli_open)=get_opening_deli(&token) {
      let frame=(deli_open,vec![TokenTree::Token(token)]);
      stack.push(frame);
      continue;
    }

    if let Some(deli_close)=get_closing_deli(&token) {
      let (deli_open,mut tree)=match stack.pop() {
        None=> return Err(Error::new(ErrorKind::UnexpectedClosingDelimiter(deli_close),token.span)),
        Some((deli_open,tree)) if deli_open==deli_close=> (deli_open,tree),
        Some(_)=> {
          let kind=ErrorKind::MismatchedClosingDelimiter(deli_close);
          return Err(Error::new(kind,token.span));
        }
      };

      tree.push(TokenTree::Token(token));
      let tt=TokenTree::Group(Group::new(deli_open,TokenStream::new(tree)));
      match stack.last_mut() {
        Some((_,last1))=> last1.push(tt),
        None=> vec.push(tt),
      }

      continue;
    }

    match stack.last_mut() {
      None=> vec.push(TokenTree::Token(token)),
      Some((_,last))=> last.push(TokenTree::Token(token)),
    }
  }


  Ok(TokenStream::new(vec))
}










#[inline]
const fn get_closing_deli(token: &Token)-> Option<Delimiter> {
  match token.kind {
    TokenKind::RParen=> Some(Delimiter::Paren),
    TokenKind::RBrace=> Some(Delimiter::Brace),
    TokenKind::RBracket=> Some(Delimiter::Bracket),
    _=> None
  }
}

#[inline]
const fn get_opening_deli(token: &Token)-> Option<Delimiter> {
  match token.kind {
    TokenKind::LParen=> Some(Delimiter::Paren),
    TokenKind::LBrace=> Some(Delimiter::Brace),
    TokenKind::LBracket=> Some(Delimiter::Bracket),
    _=> None
  }
}







