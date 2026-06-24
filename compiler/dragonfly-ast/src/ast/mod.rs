#![allow(unused)]
pub mod ident;

use std::ptr::NonNull;


pub struct Ast<T> {
  root: Option<NodePtr<T>>
}

type NodePtr<T>=NonNull<Node<T>>;
struct Node<T> {
  child: T,
  edges: Vec<Option<NodePtr<T>>>,
}

const NODE_PAYLOAD_WIDTH_MASK: u32=0xff000000;
const UNUSED_MEMORY_MASK: u32=0x7FFFFFFF;

impl<T> Ast<T> {
  pub const fn new()-> Self {
    Self {
      root: None,
    }
  }
}





