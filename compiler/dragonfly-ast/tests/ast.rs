
use dragonfly_ast::{
  ast::ident::Ident,
  token_stream::TokenStream,
  prelude::Result,
};


static MAIN: &[u8]=include_bytes!("../../assets/main.df");


#[test]
fn ident()-> Result<()> {
  for tt in &TokenStream::parse(MAIN)? {
    if tt.is_group() {
      continue;
    }

    match Ident::parse(tt.clone()) {
      Ok(ident)=> println!("{ident:#?}"),
      Err(_)=> println!("Not Ident: {tt:#?}")
    }
  }

  Ok(())
}







