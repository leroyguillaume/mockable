use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

type MockFn<RETURN, ARGS> = dyn Fn(ARGS) -> RETURN + Send + Sync;

/// Struct that represents a function mock.
///
/// **This is supported on `feature=mock` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/mock.rs).
pub struct Mock<RETURN, ARGS = ()> {
    idx: Arc<AtomicUsize>,
    kind: MockKind<RETURN, ARGS>,
}

impl<RETURN, ARGS> Mock<RETURN, ARGS> {
    /// Creates a new `Mock` that always returns always the same result.
    pub fn always_with_args<F: Fn(usize, ARGS) -> RETURN + Send + Sync + 'static>(f: F) -> Self {
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
    pub fn once_with_args<F: Fn(ARGS) -> RETURN + Send + Sync + 'static>(f: F) -> Self {
        Self::with(vec![Box::new(f)])
    }

    /// Creates a new `Mock` that should be called several times.
    pub fn with(f: Vec<Box<dyn Fn(ARGS) -> RETURN + Send + Sync>>) -> Self {
        Self {
            idx: Arc::new(AtomicUsize::new(0)),
            kind: MockKind::CallSpecific(Arc::new(f)),
        }
    }

    /// Returns the result of the mock.
    ///
    /// # Panics
    /// Panics if the mock has been called more times than expected.
    pub fn call_with_args(&self, args: ARGS) -> RETURN {
        let idx = self.idx.fetch_add(1, Ordering::Relaxed);
        match &self.kind {
            MockKind::Always(f) => f(idx, args),
            MockKind::CallSpecific(fns) => {
                if idx >= fns.len() {
                    panic!("Mock called when it should not have been");
                }
                fns[idx](args)
            }
        }
    }

    /// Returns the number of times the mock has been called.
    pub fn count(&self) -> usize {
        self.idx.load(Ordering::Relaxed)
    }

    /// Returns the number of times the mock is expected to be called.
    ///
    /// If the mock is expected to return always the same value, `usize::MAX` is returned.
    pub fn times(&self) -> usize {
        match &self.kind {
            MockKind::Always(_) => usize::MAX,
            MockKind::CallSpecific(fns) => fns.len(),
        }
    }
}

impl<RETURN> Mock<RETURN, ()> {
    /// Creates a new `Mock` that always returns always the same result.
    pub fn always<F: Fn(usize) -> RETURN + Send + Sync + 'static>(f: F) -> Self {
        Self::always_with_args(move |idx, _| f(idx))
    }

    /// Creates a new `Mock` that should be called only once.
    pub fn once<F: Fn() -> RETURN + Send + Sync + 'static>(f: F) -> Self {
        Self::once_with_args(move |_| f())
    }

    /// Returns the result of the mock.
    ///
    /// # Panics
    /// Panics if the mock has been called more times than expected.
    pub fn call(&self) -> RETURN {
        self.call_with_args(())
    }
}

impl<RETURN, ARGS> Clone for Mock<RETURN, ARGS> {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx.clone(),
            kind: self.kind.clone(),
        }
    }
}

impl<RETURN, ARGS> Default for Mock<RETURN, ARGS> {
    fn default() -> Self {
        Self::never()
    }
}

// MockKind

enum MockKind<RETURN, ARGS> {
    Always(Arc<Box<dyn Fn(usize, ARGS) -> RETURN + Send + Sync>>),
    CallSpecific(Arc<Vec<Box<MockFn<RETURN, ARGS>>>>),
}

impl<RETURN, ARGS> Clone for MockKind<RETURN, ARGS> {
    fn clone(&self) -> Self {
        match self {
            MockKind::Always(f) => MockKind::Always(f.clone()),
            MockKind::CallSpecific(fns) => MockKind::CallSpecific(fns.clone()),
        }
    }
}
