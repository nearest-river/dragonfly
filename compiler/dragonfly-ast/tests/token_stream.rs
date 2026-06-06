
use dragonfly_ast::prelude::TokenStream;


static MAIN: &[u8]=include_bytes!("../../assets/main.df");
static CHARS: &[u8]=include_bytes!("../../assets/chars.df");
static LIFETIMES: &[u8]=include_bytes!("../../assets/lifetimes.df");

#[test]
fn main() {
  test_lexer(MAIN);
}

#[test]
fn chars() {
  test_lexer(CHARS);
}

#[test]
fn lifetimes() {
  test_lexer(LIFETIMES);
}

fn test_lexer(buf: &[u8]) {
  println!("{:#?}",TokenStream::parse(buf).unwrap());
}










