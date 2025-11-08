use crate::error::Result;
use sdd::{AtomicShared, Guard, Shared, Tag};
use std::{
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};

const NODE_VERSION: &str = clap::crate_version!();
const NODE_VERSION_UNMARKED: &str = "0.2.14";

pub const NODE_RECORD_MESSAGE_VERSION: &str = "1.0.0";

pub fn get_version() -> semver::Version {
    semver::Version::parse(NODE_VERSION).expect("Failed to parse node crate version")
}

pub fn get_unmarked_version() -> semver::Version {
    semver::Version::parse(NODE_VERSION_UNMARKED).expect("Failed to parse unmarked node version")
}

pub struct DataVersionReader<T> {
    data: Shared<T>,
    guard: Guard,
}

impl<T> Deref for DataVersionReader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.get_guarded_ref(&self.guard)
    }
}

unsafe impl<T> Send for DataVersionReader<T> {}
unsafe impl<T> Sync for DataVersionReader<T> {}

impl<T> DataVersionReader<T> {
    pub fn new(atomic: &AtomicShared<T>) -> Option<Self> {
        let guard = Guard::new();
        let data = atomic.get_shared(Ordering::Acquire, &guard)?;

        Some(Self { data, guard })
    }

    /// This is only safe if the atomic is guaranteed to not be empty.
    pub fn new_unchecked(atomic: &AtomicShared<T>) -> Self {
        let guard = Guard::new();
        let data = atomic
            .get_shared(Ordering::Acquire, &guard)
            .expect("to not be empty");

        Self { data, guard }
    }

    pub fn read_field_unchecked<R, Q>(atomic: &AtomicShared<T>, func: R) -> Q
    where
        R: FnOnce(DataVersionReader<T>) -> Q,
    {
        let guard = Guard::new();
        let data = atomic
            .get_shared(Ordering::Acquire, &guard)
            .expect("to not be empty");
        func(Self { data, guard })
    }

    pub fn reader_unchecked<R>(atomic: &AtomicShared<T>, func: R)
    where
        R: FnOnce(DataVersionReader<T>),
    {
        let guard = Guard::new();
        let data = atomic
            .get_shared(Ordering::Acquire, &guard)
            .expect("to not be empty");
        func(Self { data, guard });
    }
}

pub struct DataVersionWriter<'a, T: Clone + 'static> {
    atomic: &'a AtomicShared<T>,
    new_value: T,
}

impl<T: Clone + 'static> Deref for DataVersionWriter<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.new_value
    }
}

impl<T: Clone + 'static> DerefMut for DataVersionWriter<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_value
    }
}

impl<'a, T: Clone + 'static> DataVersionWriter<'a, T> {
    pub fn new(atomic: &'a AtomicShared<T>) -> Option<Self> {
        let guard = Guard::new();
        let data = atomic.get_shared(Ordering::Acquire, &guard)?;

        Some(Self {
            atomic,
            new_value: (*data).clone(),
        })
    }

    /// This is only safe if the atomic is guaranteed to not be empty.
    pub fn new_unchecked(atomic: &'a AtomicShared<T>) -> Self {
        let guard = Guard::new();
        let data = atomic
            .get_shared(Ordering::Acquire, &guard)
            .expect("to not be empty");

        Self {
            atomic,
            new_value: (*data).clone(),
        }
    }

    pub fn from_value(atomic: &'a AtomicShared<T>, value: T) -> Self {
        Self {
            atomic,
            new_value: value,
        }
    }

    /// One shot function to create a writer and commit to the new value.
    pub fn store(atomic: &'a AtomicShared<T>, value: T) {
        let writer = Self::from_value(atomic, value);
        writer.commit();
    }

    /// One shot function to create a writer and commit a null value.
    pub fn store_null(atomic: &'a AtomicShared<T>) {
        let guard = Guard::new();
        let current = atomic.load(Ordering::Acquire, &guard);

        loop {
            let success = atomic
                .compare_exchange(
                    current,
                    (None, Tag::None),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                    &guard,
                )
                .is_ok();
            if success {
                break;
            }
        }
    }

    /// Clones the inner value.
    pub fn clone_value(&self) -> T {
        self.new_value.clone()
    }

    /// Save the current value. Retries under high contention until successful.
    pub fn commit(&self) {
        self.commit_with(|_, new_value| Shared::new(new_value));
    }

    /// Under high contention (many writers), this may fail to commit, requiring retries.
    /// To alleviate this concern, this method takes a closure to compute the new value
    /// based on the current value and the new value, reducing the window for contention.
    pub fn commit_with(&self, mut f: impl FnMut(&Option<Shared<T>>, T) -> Shared<T>) {
        self.try_commit_with(|a, b| Ok(f(a, b)))
            .expect("to not fail");
    }

    /// Same as commit_with except it returns a Result if the closure fails
    pub fn try_commit_with(
        &self,
        mut f: impl FnMut(&Option<Shared<T>>, T) -> Result<Shared<T>>,
    ) -> Result<()> {
        let guard = Guard::new();
        let mut current = self.atomic.load(Ordering::Acquire, &guard);
        let mut new = f(&current.get_shared(), self.new_value.clone())?;

        // Retry the update if it fails
        loop {
            match self.atomic.compare_exchange(
                current,
                (Some(new), Tag::None),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(_) => break,
                Err(e) => {
                    tracing::warn!("failed try_commit_with, trying again");
                    current = e.1;
                    new = f(&e.1.get_shared(), self.new_value.clone())?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    #[should_panic]
    fn upper_limit_cache() {
        let mut guards = Vec::with_capacity(u32::MAX as usize + 1);
        let data = AtomicShared::new(vec![1usize, 2, 3, 4, 5]);

        for _ in 0..(u32::MAX as usize + 2) {
            let guard = DataVersionReader::new_unchecked(&data);
            guards.push(guard);
        }
    }

    #[test]
    #[ignore]
    fn upper_limit() {
        let data = AtomicShared::new(vec![1usize, 2, 3, 4, 5]);

        for i in 0..((u32::MAX / 2) as usize + 1) {
            let _guard = DataVersionReader::new_unchecked(&data);
            let _value = DataVersionReader::read_field_unchecked(&data, |d| d[i % 5]);
        }
    }
}
