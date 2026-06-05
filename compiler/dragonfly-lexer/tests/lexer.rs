
use dragonfly_lexer::Lexer;

static MAIN: &[u8]=include_bytes!("../assets/main.df");
static CHARS: &[u8]=include_bytes!("../assets/chars.df");
static LIFETIMES: &[u8]=include_bytes!("../assets/lifetimes.df");

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
  let lexer=Lexer::new(buf);

  for token in lexer {
    println!("{token:#?}");
  }
}



