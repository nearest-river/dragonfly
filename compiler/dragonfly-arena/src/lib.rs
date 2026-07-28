#![feature(dropck_eyepatch)]

mod chunk;
pub mod marker;
mod typed_arena;
mod dropless_arena;


pub use typed_arena::TypedArena;
pub use dropless_arena::DroplessArena;

pub(crate) const PAGE_SIZE: usize=0x1000;
pub(crate) const HUGE_PAGE_SIZE: usize=0x200000;



