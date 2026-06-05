
use dragonfly_lexer::Lexer;

static CODE: &[u8]=include_bytes!("../assets/main.df");


#[test]
fn lexer() {
  let lexer=Lexer::new(CODE);

  for token in lexer {
    println!("{token:#?}");
  }
}





