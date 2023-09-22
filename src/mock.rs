use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Struct that represents a function mock.
///
/// **This is supported on `feature=mock` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/mock.rs).
pub struct Mock<E> {
    idx: Arc<AtomicUsize>,
    kind: MockKind<E>,
}

impl<E> Mock<E> {
    /// Creates a new `Mock` that always returns always the same result.
    pub fn always<F: Fn(usize) -> E + Send + Sync + 'static>(f: F) -> Self {
        Self {
            idx: Arc::new(AtomicUsize::new(0)),
            kind: MockKind::Always(Arc::new(Box::new(f))),
        }
    }

    /// Creates a new `Mock` that should never be called.
    pub fn never() -> Self {
        Self::with(vec![])
    }

    /// Creates a new `Mock` that should be called only once.
    pub fn once<F: Fn() -> E + Send + Sync + 'static>(f: F) -> Self {
        Self::with(vec![Box::new(f)])
    }

    /// Creates a new `Mock` that should be called several times.
    pub fn with(f: Vec<Box<dyn Fn() -> E + Send + Sync>>) -> Self {
        Self {
            idx: Arc::new(AtomicUsize::new(0)),
            kind: MockKind::CallSpecific(Arc::new(f)),
        }
    }

    /// Returns the result of the mock.
    ///
    /// # Panics
    /// Panics if the mock has been called more times than expected.
    pub fn call(&self) -> E {
        let idx = self.idx.fetch_add(1, Ordering::Relaxed);
        match &self.kind {
            MockKind::Always(f) => f(idx),
            MockKind::CallSpecific(fns) => {
                if idx >= fns.len() {
                    panic!("Mock called when it should not have been");
                }
                fns[idx]()
            }
        }
    }

    /// Returns the number of times the mock has been called.
    pub fn count(&self) -> usize {
        self.idx.load(Ordering::Relaxed)
    }
}

impl<E> Clone for Mock<E> {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx.clone(),
            kind: self.kind.clone(),
        }
    }
}

impl<E> Default for Mock<E> {
    fn default() -> Self {
        Self::never()
    }
}

// MockKind

enum MockKind<E> {
    Always(Arc<Box<dyn Fn(usize) -> E + Send + Sync>>),
    CallSpecific(Arc<Vec<Box<dyn Fn() -> E + Send + Sync>>>),
}

impl<E> Clone for MockKind<E> {
    fn clone(&self) -> Self {
        match self {
            MockKind::Always(f) => MockKind::Always(f.clone()),
            MockKind::CallSpecific(fns) => MockKind::CallSpecific(fns.clone()),
        }
    }
}
