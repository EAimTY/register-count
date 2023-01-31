#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::sync::{Arc, Weak};

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::sync::{Arc, Weak};

/// A counter that can be used to count the number of `Register`s referencing it.
#[derive(Clone)]
pub struct Counter(Arc<()>);

/// A register.
#[derive(Clone)]
pub struct Register(Weak<()>);

impl Counter {
    /// Create a new counter.
    pub fn new() -> Self {
        Self(Arc::new(()))
    }

    /// Create a new register referencing this counter.
    pub fn reg(&self) -> Register {
        Register(Arc::downgrade(&self.0))
    }

    /// Get the number of registers referencing this counter.
    pub fn count(&self) -> usize {
        Arc::weak_count(&self.0)
    }
}
