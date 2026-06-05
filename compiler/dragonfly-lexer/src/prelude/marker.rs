
use std::{
  rc::Rc,
  marker::PhantomData,
  panic::{
    UnwindSafe,
    RefUnwindSafe,
  },
  fmt::{
    self,
    Debug,
    Formatter,
  },
};


// Zero sized marker with the correct set of autotrait impls we want all proc
// macro types to have.
#[derive(Copy,Clone,PartialEq,Eq,Default,Hash,PartialOrd,Ord)]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);

pub(crate) const MARKER: ProcMacroAutoTraits=ProcMacroAutoTraits(PhantomData);

impl UnwindSafe for ProcMacroAutoTraits {}
impl RefUnwindSafe for ProcMacroAutoTraits {}
impl Debug for ProcMacroAutoTraits {
  fn fmt(&self,_: &mut Formatter<'_>)-> fmt::Result {
    Ok(())
  }
}



