
use std::hint;
use dragonfly_lexer::Lexer;
use criterion::{
  Criterion,
  criterion_main,
  criterion_group,
};


static SOURCE_TXT: &[u8]=hint::black_box(include_bytes!("../../assets/comments.df"));

criterion_group!(benches,parse_comments);
criterion_main!(benches);



fn parse_comments(c: &mut Criterion) {
  c.bench_function("parse-comments",|bencher| bencher.iter(|| {
    for _ in Lexer::new(SOURCE_TXT) {
    }
  }));
}



