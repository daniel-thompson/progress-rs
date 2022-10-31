//! A small library of extension traits to control and monitor the progress
//! of iterators.
//!
//! # Examples
//!
//! ```
//! use progress::*;
//!
//! for n in (0..27)
//!     .rate_limit(std::time::Duration::from_millis(10))
//!     .show_percent()
//! {
//!     // do something interesting...
//! }
//! ```

mod percent;
mod ratelimit;

pub use crate::percent::*;
pub use crate::ratelimit::*;

/// An extension trait for general iterators.
pub trait IteratorExt: Sized {
    /// Takes an iterator and creates a new iterator that will sleep
    /// if it called faster than `duration`, otherwise it is transparent.
    ///
    /// The new iterator will only sleeps when the original iterator produces
    /// a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{Duration, Instant};
    /// use progress::*;
    ///
    /// let now = Instant::now();
    /// for i in (0..10).rate_limit(Duration::from_millis(10)) {}
    /// assert!(now.elapsed() > Duration::from_millis(90));
    /// ```
    fn rate_limit(self, duration: std::time::Duration) -> RateLimitIterator<Self>;
}

impl<Iter> IteratorExt for Iter
where
    Iter: Iterator,
{
    fn rate_limit(self, duration: std::time::Duration) -> RateLimitIterator<Self> {
        RateLimitIterator::new(self, duration)
    }
}

/// An extension trait for bounded iterators.
pub trait ExactSizeIteratorExt: Sized {
    /// Takes an bounded iterator and creates a new iterator that prints
    /// a progress bar showing how much of the iterator has been consumed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::thread::sleep;
    /// use std::time::Duration;
    /// use progress::*;
    ///
    /// for i in (0..7).show_percent() {
    ///     sleep(Duration::from_millis(10));
    /// }
    /// ```
    fn show_percent(self) -> PercentIterator<Self>;
}

impl<Iter> ExactSizeIteratorExt for Iter
where
    Iter: ExactSizeIterator,
{
    fn show_percent(self) -> PercentIterator<Self> {
        PercentIterator::new(self)
    }
}
