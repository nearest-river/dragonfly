
use std::cmp::Ordering;


/// Trait for marking a type as having a sort order that is
/// stable across compilation session boundaries. More formally:
///
/// ```txt
/// Ord::cmp(a1, b1) == Ord::cmp(a2, b2)
///    where a2 = decode(encode(a1, context1), context2)
///          b2 = decode(encode(b1, context1), context2)
/// ```
///
/// i.e. the result of `Ord::cmp` is not influenced by encoding
/// the values in one session and then decoding them in another
/// session.
///
/// This is trivially true for types where encoding and decoding
/// don't change the bytes of the values that are used during
/// comparison and comparison only depends on these bytes (as
/// opposed to some non-local state). Examples are u32, String,
/// Path, etc.
///
/// But it is not true for:
///  - `*const T` and `*mut T` because the values of these pointers
///    will change between sessions.
///  - `DefIndex`, `CrateNum`, `LocalDefId`, because their concrete
///    values depend on state that might be different between
///    compilation sessions.
///
/// The associated constant `CAN_USE_UNSTABLE_SORT` denotes whether
/// unstable sorting can be used for this type. Set to true if and
/// only if `a == b` implies `a` and `b` are fully indistinguishable.
pub trait StableOrd: Ord {
  const CAN_USE_UNSTABLE_SORT: bool;

  /// Marker to ensure that implementors have carefully considered
  /// whether their `Ord` implementation obeys this trait's contract.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ();
}

impl<T: StableOrd> StableOrd for &T {
  const CAN_USE_UNSTABLE_SORT: bool=T::CAN_USE_UNSTABLE_SORT;

  // Ordering of a reference is exactly that of the referent, and since
  // the ordering of the referet is stable so must be the ordering of the
  // reference.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}

/// This is a companion trait to `StableOrd`. Some types like `Symbol` can be
/// compared in a cross-session stable way, but their `Ord` implementation is
/// not stable. In such cases, a `StableOrd` implementation can be provided
/// to offer a lightweight way for stable sorting. (The more heavyweight option
/// is to sort via `ToStableHashKey`, but then sorting needs to have access to
/// a stable hashing context and `ToStableHashKey` can also be expensive as in
/// the case of `Symbol` where it has to allocate a `String`.)
///
/// See the documentation of [StableOrd] for how stable sort order is defined.
/// The same definition applies here. Be careful when implementing this trait.
pub trait StableCompare {
  const CAN_USE_UNSTABLE_SORT: bool;
  fn stable_cmp(&self,other: &Self)-> Ordering;
}

/// `StableOrd` denotes that the type's `Ord` implementation is stable, so
/// we can implement `StableCompare` by just delegating to `Ord`.
impl<T: StableOrd> StableCompare for T {
  const CAN_USE_UNSTABLE_SORT: bool=T::CAN_USE_UNSTABLE_SORT;

  fn stable_cmp(&self,other: &Self)-> Ordering {
    self.cmp(other)
  }
}

impl<T1: StableOrd,T2: StableOrd> StableOrd for (T1,T2) {
  const CAN_USE_UNSTABLE_SORT: bool=T1::CAN_USE_UNSTABLE_SORT && T2::CAN_USE_UNSTABLE_SORT;

  // Ordering of tuples is a pure function of their elements' ordering, and since
  // the ordering of each element is stable so must be the ordering of the tuple.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}

impl<T1: StableOrd,T2: StableOrd,T3: StableOrd> StableOrd for (T1,T2,T3) {
  const CAN_USE_UNSTABLE_SORT: bool=T1::CAN_USE_UNSTABLE_SORT && T2::CAN_USE_UNSTABLE_SORT && T3::CAN_USE_UNSTABLE_SORT;

  // Ordering of tuples is a pure function of their elements' ordering, and since
  // the ordering of each element is stable so must be the ordering of the tuple.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}


impl<T1: StableOrd, T2: StableOrd, T3: StableOrd, T4: StableOrd> StableOrd for (T1,T2,T3,T4) {
  const CAN_USE_UNSTABLE_SORT: bool=T1::CAN_USE_UNSTABLE_SORT && T2::CAN_USE_UNSTABLE_SORT && T3::CAN_USE_UNSTABLE_SORT && T4::CAN_USE_UNSTABLE_SORT;
  // Ordering of tuples is a pure function of their elements' ordering, and since
  // the ordering of each element is stable so must be the ordering of the tuple.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}


impl StableOrd for &str {
  const CAN_USE_UNSTABLE_SORT: bool=true;

  // Encoding and decoding doesn't change the bytes of string slices
  // and `Ord::cmp` depends only on those bytes.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}


impl StableOrd for String {
  const CAN_USE_UNSTABLE_SORT: bool=true;

  // String comparison only depends on their contents and the
  // contents are not changed by (de-)serialization.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}

impl StableOrd for bool {
  const CAN_USE_UNSTABLE_SORT: bool=true;

  // sort order of bools is not changed by (de-)serialization.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}

impl<T: StableOrd> StableOrd for Option<T> {
  const CAN_USE_UNSTABLE_SORT: bool=T::CAN_USE_UNSTABLE_SORT;

  // the Option wrapper does not add instability to comparison.
  const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED: ()=();
}






