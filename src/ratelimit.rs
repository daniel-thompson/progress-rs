use std::time::{Duration, Instant};

/// A simple never-faster-than-the-interval rate limiter.
///
/// # Examples
///
/// Skips an action if if happens too frequently (in this case
/// we expect to skip every iteration except the first):
///
/// ```
/// use std::time::Duration;
/// use progress::RateLimit;
///
/// let mut limiter = RateLimit::new(Duration::from_secs(5));
/// let mut total = 0;
/// for i in 3..10 {
///     limiter.act(|| total += i);
/// }
/// assert_eq!(total, 3);
/// ```
///
/// Alternatively we can sleep until we are ready to go again:
///
/// ```
/// use std::time::{Duration, Instant};
/// use progress::RateLimit;
///
/// let mut limiter = RateLimit::new(Duration::from_millis(10));
/// let now = Instant::now();
/// for i in 0..10 {
///     limiter.sleep_act(|| ());
/// }
/// assert!(now.elapsed() > Duration::from_millis(90))
/// ```
///
#[derive(Debug)]
pub struct RateLimit {
    interval: Duration,
    last: Instant,
}

impl RateLimit {
    /// Initialize a rate limiter for the specified interval.
    ///
    /// For examples, see [`crate::RateLimit`].
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last: Instant::now() - interval,
        }
    }

    /// Attempt to run an action and report whether or not we skipped the
    /// action.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use progress::RateLimit;
    ///
    /// let mut limiter = RateLimit::new(Duration::from_secs(5));
    /// let mut total = 0;
    /// let mut skipped = 0;
    /// for i in 0..10 {
    ///     match limiter.try_act(|| 100) {
    ///     Some(v) => total += v,
    ///     None => skipped += 1,
    ///     }
    /// }
    /// assert_eq!(total, 100);
    /// assert_eq!(skipped, 9);
    /// ```
    pub fn try_act<T>(&mut self, f: impl FnOnce() -> T) -> Option<T> {
        if self.last.elapsed() >= self.interval {
            self.last = Instant::now();
            Some(f())
        } else {
            None
        }
    }

    /// Attempt to run an action, skipping it if we hit the rate limiter.
    ///
    /// Unlike other methods from the act family, the closure provided *must*
    /// return `()` because we cannot know the right value to return it we
    /// skip the action.
    ///
    /// For examples, see [`crate::RateLimit`].
    pub fn act(&mut self, f: impl FnOnce() -> ()) {
        self.try_act(f);
    }

    /// Run the action, sleeping until the rate limit has clears if necessary.
    ///
    /// For examples, see [`crate::RateLimit`].
    pub fn sleep_act<T>(&mut self, f: impl FnOnce() -> T) -> T {
        let elapsed = self.last.elapsed();
        if elapsed < self.interval {
            std::thread::sleep(self.interval - elapsed);
        }

        self.last += self.interval;
        f()
    }
}

/// Wraps an iterator and sleeps if it called faster than `duration`,
/// otherwise it is transparent.
///
/// Typically created using the [`crate::IteratorExt::rate_limit()`] method.
#[derive(Debug)]
pub struct RateLimitIterator<Iter> {
    iter: Iter,
    ratelimit: RateLimit,
}

impl<Iter> RateLimitIterator<Iter> {
    /// Directly wrap an iterator and rate limit it.
    ///
    /// In most cases it is better to use [`crate::IteratorExt::rate_limit()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{Duration, Instant};
    /// use progress::*;
    ///
    /// let now = Instant::now();
    /// for i in RateLimitIterator::new((0..10), Duration::from_millis(10)) {}
    /// assert!(now.elapsed() > Duration::from_millis(90));
    /// ```
    pub fn new(iter: Iter, duration: Duration) -> Self {
        RateLimitIterator {
            iter,
            ratelimit: RateLimit::new(duration),
        }
    }
}

impl<Iter> Iterator for RateLimitIterator<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|s| self.ratelimit.sleep_act(|| s))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<Iter> ExactSizeIterator for RateLimitIterator<Iter> where Iter: ExactSizeIterator {}
