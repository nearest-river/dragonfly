
use dragonfly_lexer::Lexer;

static MAIN: &[u8]=include_bytes!("../assets/main.df");
static CHARS: &[u8]=include_bytes!("../assets/chars.df");


fn lexer() {
  let lexer=Lexer::new(MAIN);

  for token in lexer {
    println!("{token:#?}");
  }
}

#[test]
fn chars() {
  let lexer=Lexer::new(CHARS);

  for token in lexer {
    println!("{token:#?}");
  }
}





