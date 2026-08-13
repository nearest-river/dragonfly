
use dragonfly_ast::prelude::TokenStream;


static MAIN: &[u8]=include_bytes!("../../assets/main.df");
static CHARS: &[u8]=include_bytes!("../../assets/chars.df");
static LIFETIMES: &[u8]=include_bytes!("../../assets/lifetimes.df");

#[test]
fn main() {
  test_parser(MAIN);
}

#[test]
fn chars() {
  test_parser(CHARS);
}

#[test]
fn lifetimes() {
  test_parser(LIFETIMES);
}

fn test_parser(buf: &[u8]) {
  println!("{:#?}",TokenStream::parse(buf).unwrap());
}










