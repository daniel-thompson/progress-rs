use std::io::{stdout, Write};
use std::time::Duration;

use crate::ratelimit::*;

const INTERVAL: Duration = Duration::from_millis(100);

/// Wraps an bounded iterator and prints a progress bar showing how
/// much of the iterator has been consumed.
///
/// Typically created using the
/// [`crate::ExactSizeIteratorExt::show_percent()`] method.
#[derive(Debug)]
pub struct PercentIterator<Iter> {
    iter: Iter,
    bound: usize,
    ratelimit: RateLimit,
}

impl<Iter> PercentIterator<Iter>
where
    Iter: ExactSizeIterator,
{
    /// Directly wrap a bounded iterator and print a progress bar.
    ///
    /// In most cases it is better to use
    /// [`crate::ExactSizeIteratorExt::show_percent()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use progress::*;
    ///
    /// for i in PercentIterator::new((0..7)) {}
    /// ```
    pub fn new(iter: Iter) -> Self {
        let bound = iter.len();
        PercentIterator {
            iter,
            bound,
            ratelimit: RateLimit::new(INTERVAL),
        }
    }
}

impl<Iter> Iterator for PercentIterator<Iter>
where
    Iter: ExactSizeIterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.len() {
            len if len != 0 => self.ratelimit.act(|| {
                let bound = self.bound as f64;
                let percent = 100.0 * (bound - len as f64) / bound;
                let bar = (percent / 2.0) as usize;

                print!(
                    "\r|{}{}| {percent:5.1}%",
                    "#".repeat(bar),
                    " ".repeat(50 - bar)
                );
                stdout().flush().expect("failed to flush stdout");
            }),
            _ => println!("\r|##################################################| 100.0%"),
        };

        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<Iter> ExactSizeIterator for PercentIterator<Iter> where Iter: ExactSizeIterator {}
