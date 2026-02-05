// core/timed.rs

use std::time::{Duration, Instant};

/// Trait implemented by all timed return values.
/// Allows generic handling of elapsed time.
pub trait HasElapsed {
    fn elapsed(&self) -> Duration;
}

/// Used when a function ALWAYS succeeds.
#[derive(Debug)]
pub struct Timed<T> {
    pub value: T,
    pub elapsed: Duration,
}

impl<T> Timed<T> {
    /// Measure a computation that cannot fail.
    pub fn measure<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let value = f();
        Self {
            value,
            elapsed: start.elapsed(),
        }
    }

    /// Map the contained value while preserving timing.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Timed<U> {
        Timed {
            value: f(self.value),
            elapsed: self.elapsed,
        }
    }
}

impl<T> HasElapsed for Timed<T> {
    fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

/// Used when a function MAY fail.
#[derive(Debug)]
pub struct TimedResult<T, E> {
    pub value: Result<T, E>,
    pub elapsed: Duration,
}

impl<T, E> TimedResult<T, E> {
    /// Measure a computation that may fail.
    pub fn measure<F>(f: F) -> Self
    where
        F: FnOnce() -> Result<T, E>,
    {
        let start = Instant::now();
        let value = f();
        Self {
            value,
            elapsed: start.elapsed(),
        }
    }

    /// Check success.
    pub fn is_ok(&self) -> bool {
        self.value.is_ok()
    }

    /// Check failure.
    pub fn is_err(&self) -> bool {
        self.value.is_err()
    }

    /// Extract success value.
    pub fn ok(self) -> Option<T> {
        self.value.ok()
    }

    /// Extract error value.
    pub fn err(self) -> Option<E> {
        self.value.err()
    }

    /// Map success value while preserving timing.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> TimedResult<U, E> {
        TimedResult {
            value: self.value.map(f),
            elapsed: self.elapsed,
        }
    }

    /// Convert into a normal Result (dropping timing).
    pub fn into_result(self) -> Result<T, E> {
        self.value
    }
}

impl<T, E> HasElapsed for TimedResult<T, E> {
    fn elapsed(&self) -> Duration {
        self.elapsed
    }
}
