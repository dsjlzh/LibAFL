//! The `CmpObserver` provides access to the logged values of CMP instructions

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::slice::from_raw_parts_mut;
use serde::{Deserialize, Serialize};

use crate::{
    bolts::{
        ownedref::{OwnedRefMut, OwnedSliceMut},
        tuples::Named,
    },
    executors::HasExecHooks,
    observers::Observer,
    Error,
};

/// A [`MapObserver`] observes the static map, as oftentimes used for afl-like coverage information
pub trait CmpObserver<T>: Observer
{
    /// Get the number of usable cmps (all by default)
    fn usable_count(&self) -> usize;

    /// Get the number of cmps
    fn len(&self) -> usize;

    fn executions_for(idx: usize) -> usize;
    
    fn kind_for(idx: usize) -> CmpKind;

    fn bytes_for(idx: usize) -> usize;

    fn values_of(idx: usize, execution: usize) -> (T, T);

    /// Reset the state
    fn reset(&mut self) -> Result<(), Error>;
}

