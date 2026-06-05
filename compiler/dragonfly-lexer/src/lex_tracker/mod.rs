
mod num;
mod rident;
mod comment;
mod quotable;

pub use num::*;
pub use rident::*;
pub use comment::*;
pub use quotable::*;


pub enum LexTracker {
  Other,
  Num(NumLexTracker),
  RIdent(RIdentLexTracker),
  Quotable(QuotableTracker),
}











