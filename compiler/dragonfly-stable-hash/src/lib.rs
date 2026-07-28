#![allow(incomplete_features)]
#![feature(never_type,specialization,negative_impls)]

#[cfg(test)]
mod tests;

mod hash;

pub mod cmp;
pub mod bindings;


// use rustc_index::bit_set::{self, DenseBitSet};
// use rustc_index::{Idx, IndexSlice, IndexVec};
// use smallvec::SmallVec;


// use rustc_hashes::{Hash64, Hash128};
pub use hash::*;
pub use rustc_stable_hash::{
  FromStableHash,
  StableSipHasher128 as StableHasher,
  SipHasher128Hash as StableHasherHash,
};





/// Implement StableHash by just calling `Hash::hash()`. Also implement `StableOrd` for the type
/// since that has the same requirements.
///
/// **WARNING** This is only valid for types that *really* don't need any context for fingerprinting.
/// But it is easy to misuse this macro (see [#96013](https://github.com/rust-lang/rust/issues/96013)
/// for examples). Therefore this macro is not exported and should only be used in the limited cases
/// here in this module.
///
/// Use `#[derive(StableHash)]` instead.
macro_rules! impl_stable_traits_for_trivial_type {
  ($t:ty) => {
    impl $crate::StableHash for $t {
      #[inline]
      fn stable_hash<Hcx>(&self,_: &mut Hcx,hasher: &mut $crate::StableHasher) {
        ::std::hash::Hash::hash(self,hasher);
      }
    }

    impl $crate::cmp::StableOrd for $t {
      const CAN_USE_UNSTABLE_SORT: bool=true;

      // Encoding and decoding doesn't change the bytes of trivial types
      // and `Ord::cmp` depends only on those bytes.
      const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
    }
  };
}

pub(crate) use impl_stable_traits_for_trivial_type;
impl_stable_traits_for_trivial_type!(i8);
impl_stable_traits_for_trivial_type!(i16);
impl_stable_traits_for_trivial_type!(i32);
impl_stable_traits_for_trivial_type!(i64);
impl_stable_traits_for_trivial_type!(isize);

impl_stable_traits_for_trivial_type!(u8);
impl_stable_traits_for_trivial_type!(u16);
impl_stable_traits_for_trivial_type!(u32);
impl_stable_traits_for_trivial_type!(u64);
impl_stable_traits_for_trivial_type!(usize);

impl_stable_traits_for_trivial_type!(u128);
impl_stable_traits_for_trivial_type!(i128);

impl_stable_traits_for_trivial_type!(char);
impl_stable_traits_for_trivial_type!(());

// impl_stable_traits_for_trivial_type!(Hash64);




















