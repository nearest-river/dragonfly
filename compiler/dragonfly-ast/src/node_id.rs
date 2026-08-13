
use dragonfly_stable_hash::*;
use std::fmt::{
  self,
  Display,
  Formatter,
};


dragonfly_index::newtype_index! {
  /// Identifies an AST node.
  ///
  /// This identifies top-level definitions, expressions, and everything in between.
  /// This is later turned into [`DefId`] and `HirId` for the HIR.
  ///
  /// [`DefId`]: dragonfly_span::def_id::DefId
  #[orderable]
  #[debug_format="NodeId({})"]
  pub struct NodeId {
    /// The [`NodeId`] used to represent the root of the crate.
    const CRATE_NODE_ID=0;
  }
}


pub const DUMMY_NODE_ID: NodeId=NodeId::MAX;


impl StableHash for NodeId {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,_: &mut Hcx,_: &mut StableHasher) {
    // This impl is never called but is necessary for types implementing `StableHash` such as
    // `MainDefinition` and `DocLinkResMap` (both of which occur in `ResolverGlobalCtxt`).
    panic!("Node IDs should not appear in incremental state");
  }
}


impl Display for NodeId {
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    Display::fmt(&self.as_u32(),f)
  }
}


