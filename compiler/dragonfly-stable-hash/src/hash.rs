
use crate::{
  bindings::*,
  StableHasher,
  cmp::StableOrd,
  impl_stable_traits_for_trivial_type,
};

use std::{
  rc::Rc,
  sync::Arc,
  ffi::OsStr,
  num::NonZero,
  cmp::Ordering,
  collections::*,
  ops::RangeInclusive,
  marker::PhantomData,
  mem::{
    self,
    Discriminant,
  },
  path::{
    Path,
    PathBuf,
  },
  hash::{
    Hash,
    Hasher,
  },
};




/// This trait lets `StableHash` and `derive(StableHash)` be used in
/// this crate (and other crates upstream of `rustc_middle`),while leaving
/// certain operations to be defined in `rustc_middle` where more things are
/// visible.
pub trait StableHashCtxt {
  /// The main event: stable hashing of a span.
  fn stable_hash_span(&mut self,span: RawSpan,hasher: &mut StableHasher);
  /// Compute a `DefPathHash`.
  fn def_path_hash(&self,def_id: RawDefId)-> RawDefPathHash;
  /// Get the stable hash controls.
  fn stable_hash_controls(&self)-> StableHashControls;
  /// Assert that the provided `StableHashCtxt` is configured with the default
  /// `StableHashControls`. We should always have bailed out before getting to here with a
  fn assert_default_stable_hash_controls(&self,msg: &str);
}



/// Something that implements `StableHash` can be hashed in a way that is
/// stable across multiple compilation sessions.
///
/// Note that `StableHash` imposes rather more strict requirements than usual
/// hash functions:
///
/// - Stable hashes are sometimes used as identifiers. Therefore they must
///   conform to the corresponding `PartialEq` implementations:
///
///     - `x == y` implies `stable_hash(x) == stable_hash(y)`,and
///     - `x != y` implies `stable_hash(x) != stable_hash(y)`.
///
///   That second condition is usually not required for hash functions
///   (e.g. `Hash`). In practice this means that `stable_hash` must feed any
///   information into the hasher that a `PartialEq` comparison takes into
///   account. See [#49300](https://github.com/rust-lang/rust/issues/49300)
///   for an example where violating this invariant has caused trouble in the
///   past.
///
/// - `stable_hash()` must be independent of the current
///    compilation session. E.g. they must not hash memory addresses or other
///    things that are "randomly" assigned per compilation session.
///
/// - `stable_hash()` must be independent of the host architecture. The
///   `StableHasher` takes care of endianness and `isize`/`usize` platform
///   differences.
pub trait StableHash {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher);
}

/// Implement this for types that can be turned into stable keys like,for
/// example,for DefId that can be converted to a DefPathHash. This is used for
/// bringing maps into a predictable order before hashing them.
pub trait ToStableHashKey {
  type KeyType: Ord+Sized+StableHash;
  fn to_stable_hash_key<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx)-> Self::KeyType;
}

/// Controls what data we do or do not hash.
/// Whenever a `StableHash` implementation caches its
/// result,it needs to include `StableHashControls` as part
/// of the key,to ensure that it does not produce an incorrect
/// result (for example,using a `Fingerprint` produced while
/// hashing `Span`s when a `Fingerprint` without `Span`s is
/// being requested)
#[derive(Clone,Copy,Hash,Eq,PartialEq,Debug)]
pub struct StableHashControls {
  pub hash_spans: bool,
}



impl StableHash for ! {
  fn stable_hash<Hcx>(&self,_hcx: &mut Hcx,_hasher: &mut StableHasher) {
    unreachable!()
  }
}

impl<T> StableHash for PhantomData<T> {
  fn stable_hash<Hcx>(&self,_hcx: &mut Hcx,_hasher: &mut StableHasher) {}
}

impl StableHash for NonZero<u32> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.get().stable_hash(hcx,hasher)
  }
}

impl StableHash for NonZero<usize> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.get().stable_hash(hcx,hasher)
  }
}

impl StableHash for f32 {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let val=self.to_bits();
    val.stable_hash(hcx,hasher);
  }
}

impl StableHash for f64 {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let val=self.to_bits();
    val.stable_hash(hcx,hasher);
  }
}

impl StableHash for Ordering {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    (*self as i8).stable_hash(hcx,hasher);
  }
}

impl<T1: StableHash> StableHash for (T1,) {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let (ref _0,)=*self;
    _0.stable_hash(hcx,hasher);
  }
}

impl<T1: StableHash,T2: StableHash> StableHash for (T1,T2) {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let (ref _0,ref _1)=*self;
    _0.stable_hash(hcx,hasher);
    _1.stable_hash(hcx,hasher);
  }
}



impl<T1: StableHash,T2: StableHash,T3: StableHash> StableHash for (T1,T2,T3) {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let (ref _0,ref _1,ref _2)=*self;
    _0.stable_hash(hcx,hasher);
    _1.stable_hash(hcx,hasher);
    _2.stable_hash(hcx,hasher);
  }
}



impl<T1: StableHash,T2: StableHash,T3: StableHash,T4: StableHash> StableHash for (T1,T2,T3,T4) {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let (ref _0,ref _1,ref _2,ref _3)=*self;
    _0.stable_hash(hcx,hasher);
    _1.stable_hash(hcx,hasher);
    _2.stable_hash(hcx,hasher);
    _3.stable_hash(hcx,hasher);
  }
}



impl<T: StableHash> StableHash for [T] {
  default fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for item in self {
      item.stable_hash(hcx,hasher);
    }
  }
}

impl StableHash for [u8] {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    hasher.write(self);
  }
}

impl<T: StableHash> StableHash for Vec<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self[..].stable_hash(hcx,hasher);
  }
}

/*
impl<K: StableHash+Eq+Hash,V: StableHash,R: BuildHasher> StableHash for indexmap::IndexMap<K,V,R>
where
  K: StableHash + Eq + Hash,
  V: StableHash,
  R: BuildHasher,
{
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for kv in self {
      kv.stable_hash(hcx,hasher);
    }
  }
}

impl<K,R> StableHash for indexmap::IndexSet<K,R>
where
  K: StableHash + Eq + Hash,
  R: BuildHasher,
{
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for key in self {
      key.stable_hash(hcx,hasher);
    }
  }
}

impl<A,const N: usize> StableHash for SmallVec<[A; N]>
where
  A: StableHash,
{
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self[..].stable_hash(hcx,hasher);
  }
}*/

impl<T: ?Sized + StableHash> StableHash for Box<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    (**self).stable_hash(hcx,hasher);
  }
}

impl<T: ?Sized+StableHash> StableHash for Rc<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    (**self).stable_hash(hcx,hasher);
  }
}

impl<T: ?Sized+StableHash> StableHash for Arc<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    (**self).stable_hash(hcx,hasher);
  }
}

impl StableHash for str {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.as_bytes().stable_hash(hcx,hasher);
  }
}


impl StableHash for String {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self[..].stable_hash(hcx,hasher);
  }
}


impl StableHash for bool {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    let byte=if *self { 1u8 } else { 0u8 };
    byte.stable_hash(hcx,hasher);
  }
}


impl<T: StableHash> StableHash for Option<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    if let Some(ref value)=*self {
      1u8.stable_hash(hcx,hasher);
      value.stable_hash(hcx,hasher);
    } else {
      0u8.stable_hash(hcx,hasher);
    }
  }
}


impl<T1: StableHash,T2: StableHash> StableHash for Result<T1,T2> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    mem::discriminant(self).stable_hash(hcx,hasher);
    match *self {
      Ok(ref x)=> x.stable_hash(hcx,hasher),
      Err(ref x)=> x.stable_hash(hcx,hasher),
    }
  }
}

impl<'a,T: StableHash+?Sized> StableHash for &'a T {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    (**self).stable_hash(hcx,hasher);
  }
}

impl<T> StableHash for Discriminant<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,_: &mut Hcx,hasher: &mut StableHasher) {
    Hash::hash(self,hasher);
  }
}

impl<T: StableHash> StableHash for RangeInclusive<T> {
  #[inline]
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.start().stable_hash(hcx,hasher);
    self.end().stable_hash(hcx,hasher);
  }
}

/*
impl<I: Idx,T> StableHash for IndexSlice<I,T>
where
  T: StableHash,
{
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for v in &self.raw {
      v.stable_hash(hcx,hasher);
    }
  }
}

impl<I: Idx,T> StableHash for IndexVec<I,T>
where
  T: StableHash,
{
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for v in &self.raw {
      v.stable_hash(hcx,hasher);
    }
  }
}

impl<I: Idx> StableHash for DenseBitSet<I> {
  fn stable_hash<Hcx: StableHashCtxt>(&self,_hcx: &mut Hcx,hasher: &mut StableHasher) {
    ::std::hash::Hash::hash(self,hasher);
  }
}

impl<R: Idx,C: Idx> StableHash for bit_set::BitMatrix<R,C> {
  fn stable_hash<Hcx: StableHashCtxt>(&self,_hcx: &mut Hcx,hasher: &mut StableHasher) {
    ::std::hash::Hash::hash(self,hasher);
  }
}*/

impl_stable_traits_for_trivial_type!(OsStr);

impl_stable_traits_for_trivial_type!(Path);
impl_stable_traits_for_trivial_type!(PathBuf);

// It is not safe to implement StableHash for HashSet,HashMap or any other collection type
// with unstable but observable iteration order.
// See https://github.com/rust-lang/compiler-team/issues/533 for further information.
impl<V> !StableHash for HashSet<V> {}
impl<K,V> !StableHash for HashMap<K,V> {}

impl<K: StableHash,V: StableHash+crate::cmp::StableOrd> StableHash for BTreeMap<K,V>  {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for entry in self.iter() {
      entry.stable_hash(hcx,hasher);
    }
  }
}

impl<K: StableHash+StableOrd> StableHash for BTreeSet<K> {
  fn stable_hash<Hcx: StableHashCtxt>(&self,hcx: &mut Hcx,hasher: &mut StableHasher) {
    self.len().stable_hash(hcx,hasher);
    for entry in self.iter() {
      entry.stable_hash(hcx,hasher);
    }
  }
}
















