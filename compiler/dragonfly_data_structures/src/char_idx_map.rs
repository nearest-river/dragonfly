
use crate::{
  fx::FxHashMap,
  indexable::Indexable,
  monotonic::MonotonicVec,
};

use std::{
  mem::ManuallyDrop,
  marker::PhantomData,
};



pub struct CharIndexMap<K: Indexable,V> {
  kind: MapKind,
  inner: InnerMap<K,V>,
  _marker: PhantomData<(K,V)>,
}


union InnerMap<K,V> {
  vec_map: ManuallyDrop<MonotonicVec<V>>,
  hash_map: ManuallyDrop<FxHashMap<K,V>>,
}

enum MapKind {
  VecMap,
  HashMap,
}

impl<K: Indexable,V> CharIndexMap<K,V> {
  pub const fn new()-> Self {
    let inner=InnerMap {
      vec_map: ManuallyDrop::new(MonotonicVec::new())
    };
    Self {
      inner,
      kind: MapKind::VecMap,
      _marker: PhantomData,
    }
  }
}







impl<K: Indexable,V> Drop for CharIndexMap<K,V> {
  fn drop(&mut self) {
    // SAFETY: trust me bro.
    unsafe {
      match self.kind {
        MapKind::VecMap=> ManuallyDrop::drop(&mut self.inner.vec_map),
        MapKind::HashMap=> ManuallyDrop::drop(&mut self.inner.hash_map),
      }
    }
  }
}





