
use crate::{
  marker::*,
  sync::mode,
};

use parking_lot::{
  RawMutex,
  lock_api::RawMutex as _,
};

use std::{
  hint,
  mem::ManuallyDrop,
  marker::PhantomData,
  ops::{
    Deref,
    DerefMut,
  },
  cell::{
    Cell,
    UnsafeCell,
  },
  fmt::{
    self,
    Debug,
    Formatter,
  },
};


pub struct Lock<T> {
  mode: LockMode,
  mode_union: ModeUnion,
  data: UnsafeCell<T>,
}

union ModeUnion {
  no_sync: ManuallyDrop<Cell<bool>>,
  sync: ManuallyDrop<RawMutex>,
}

/// A guard holding mutable access to a `Lock` which is in a locked state.
#[must_use="if unused the Lock will immediately unlock"]
pub struct LockGuard<'a,T> {
  lock: &'a Lock<T>,
  mode: LockMode,
  _marker: PhantomData<&'a mut T>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LockMode {
  NoSync,
  Sync,
}

/// The value representing a locked state for the `Cell`.
const LOCKED: bool=true;

impl<T> Lock<T> {
  #[inline(always)]
  pub fn new(inner: T)-> Self {
    let (mode,mode_union)=if mode::might_be_dyn_thread_safe() {
      hint::cold_path();
      // Create the lock with synchronization enabled using the `RawMutex` type.
      (LockMode::Sync,ModeUnion { sync: ManuallyDrop::new(RawMutex::INIT) })
    } else {
      (LockMode::NoSync,ModeUnion { no_sync: ManuallyDrop::new(Cell::new(!LOCKED)) })
    };

    Lock {
      mode,
      mode_union,
      data: UnsafeCell::new(inner)
    }
  }

  #[inline]
  pub fn into_inner(self)-> T {
    self.data.into_inner()
  }

  #[inline]
  pub fn get_mut(&mut self)-> &mut T {
    self.data.get_mut()
  }

  #[inline(always)]
  pub fn try_lock(&self)-> Option<LockGuard<'_,T>> {
    let mode=self.mode;

    let success=match mode {
      LockMode::Sync=> unsafe { self.mode_union.sync.try_lock() },
      LockMode::NoSync=> {
        let cell=unsafe { &self.mode_union.no_sync };
        let was_unlocked=cell.get()!=LOCKED;

        if was_unlocked {
          cell.set(LOCKED);
        }

        was_unlocked
      },
    };

    success.then(|| LockGuard {
      mode,
      lock: self,
      _marker: PhantomData,
    })
  }

  /// This acquires the lock assuming synchronization is in a specific mode.
  ///
  /// Safety
  /// This method must only be called with `Mode::Sync` if `might_be_dyn_thread_safe` was
  /// true on lock creation.
  #[track_caller]
  #[inline(always)]
  pub unsafe fn lock_assume(&self,mode: LockMode)-> LockGuard<'_,T> {
    #[cold]
    #[track_caller]
    #[inline(never)]
    fn lock_held()-> ! {
      panic!("lock was already held")
    }

    unsafe {
      match mode {
        LockMode::NoSync=> if self.mode_union.no_sync.replace(LOCKED)==LOCKED {
          hint::cold_path();
          lock_held()
        },
        LockMode::Sync=> self.mode_union.sync.lock(),
      }
    }

    LockGuard {
      mode,
      lock: self,
      _marker: PhantomData,
    }
  }

  #[track_caller]
  #[inline(always)]
  pub fn lock(&self)-> LockGuard<'_,T> {
    unsafe {
      self.lock_assume(self.mode)
    }
  }

  #[track_caller]
  #[inline(always)]
  pub fn with_lock<U,F: FnOnce(&mut T)-> U>(&self,f: F)-> U {
    f(&mut *self.lock())
  }
}

unsafe impl<T: DynSend> DynSend for Lock<T> {}
unsafe impl<T: DynSend> DynSync for Lock<T> {}

impl<T: Default> Default for Lock<T> {
  #[inline]
  fn default()-> Lock<T> {
    Lock::new(T::default())
  }
}

impl<T: Debug> Debug for Lock<T> {
  #[inline]
  fn fmt(&self,f: &mut Formatter<'_>)-> fmt::Result {
    let mut fmt=f.debug_struct(stringify!(Lock));

    match self.try_lock() {
      Some(guard)=> fmt.field("data",&*guard),
      None=> fmt.field("data",&"<locked>"),
    };

    fmt.finish()
  }
}


impl<'a,T: 'a> Deref for LockGuard<'a,T> {
  type Target=T;
  #[inline]
  fn deref(&self)-> &Self::Target {
    // SAFETY: We have shared access to the mutable access owned by this type,
    // so we can give out a shared reference.
    unsafe {
      &*self.lock.data.get()
    }
  }
}

impl<'a,T: 'a> DerefMut for LockGuard<'a,T> {
  #[inline]
  fn deref_mut(&mut self)-> &mut Self::Target {
    // SAFETY: We have mutable access to the data so we can give out a mutable reference.
    unsafe {
      &mut *self.lock.data.get()
    }
  }
}

impl<'a,T> Drop for LockGuard<'a,T> {
  fn drop(&mut self) {
    // SAFETY (union access): trust me bro.
    match self.mode {
      LockMode::Sync=> unsafe { self.lock.mode_union.sync.unlock() },
      LockMode::NoSync=> {
        let cell=unsafe { &self.lock.mode_union.no_sync };
        debug_assert!(cell.get());
        cell.set(false);
      },
    }
  }
}


