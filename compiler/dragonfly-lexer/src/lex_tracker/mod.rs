
mod num;
mod rident;
mod comment;
mod str_or_char;

pub use num::*;
pub use rident::*;
pub use comment::*;
pub use str_or_char::*;


pub enum LexTracker {
  Other,
  Num(NumLexTracker),
  RIdent(RIdentLexTracker),
  StrOrChar(StrOrCharTracker),
}











