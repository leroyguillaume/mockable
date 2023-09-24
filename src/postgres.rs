use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    ops::Deref,
    pin::Pin,
};

use async_trait::async_trait;
use deadpool_postgres::{
    tokio_postgres::{Client, Error as TokioPostgresError},
    Object, Pool, PoolError, Transaction,
};
use tracing::trace;

// Types

pub type PostgresResult<T> = Result<T, PostgresError>;

// PostgresError

/// An that can occur when interacting with Postgres.
#[derive(Debug)]
pub struct PostgresError(Box<dyn Error + Send + Sync>);

impl Display for PostgresError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for PostgresError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0.as_ref())
    }
}

impl From<PoolError> for PostgresError {
    fn from(err: PoolError) -> Self {
        Self(Box::new(err))
    }
}

impl From<TokioPostgresError> for PostgresError {
    fn from(err: TokioPostgresError) -> Self {
        Self(Box::new(err))
    }
}

// PostgresClient

/// A client for interacting with Postgres.
///
/// **This is supported on `feature=postgres` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
#[async_trait]
pub trait PostgresClient: Send + Sync + ToPostgresClient {
    /// Returns the underlying [`Client`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Client.html) instance.
    fn into_client(self: Box<Self>) -> Object;

    /// Opens a new transaction.
    ///
    /// See [`Client::transaction`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Client.html#method.transaction) for more information.
    async fn transaction(&mut self) -> PostgresResult<Box<dyn PostgresTransaction + '_>>;

    /// Returns self reference.
    fn upcast(&self) -> &dyn ToPostgresClient;
}

// PostgresPool

/// A pool of Postgres clients.
///
/// **This is supported on `feature=postgres` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
#[async_trait]
pub trait PostgresPool: Send + Sync {
    /// Returns a client from the pool.
    async fn get(&self) -> PostgresResult<Box<dyn PostgresClient>>;
}

// PostgresTransaction

/// A Postgres transaction.
///
/// **This is supported on `feature=postgres` only.**
#[async_trait]
pub trait PostgresTransaction<'a>: Send + Sync + ToPostgresClient {
    /// Commits the transaction.
    async fn commit(self: Box<Self>) -> PostgresResult<()>;

    /// Returns the underlying [`Transaction`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Transaction.html) instance.
    fn into_transaction(self: Box<Self>) -> Transaction<'a>;

    /// Rolls back the transaction.
    async fn rollback(self: Box<Self>) -> PostgresResult<()>;

    /// Returns a reference to the underlying [`Transaction`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Transaction.html) instance.
    fn to_transaction(&self) -> &Transaction<'a>;

    /// Returns self reference.
    fn upcast(&self) -> &dyn ToPostgresClient;
}

// ToPostgresClient

/// A trait for converting a type into a [`PostgresClient`](trait.PostgresClient.html).
pub trait ToPostgresClient {
    /// Returns a reference to the underlying [`Client`](https://docs.rs/tokio-postgres/latest/tokio_postgres/struct.Client.html) instance.
    fn to_client(&self) -> &Client;
}

// DefaultPostgresClient

/// Default implementation of [`PostgresClient`](trait.PostgresClient.html).
///
/// **This is supported on `feature=postgres` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
pub struct DefaultPostgresClient(Object);

impl DefaultPostgresClient {
    /// Create a new `DefaultPostgresClient`.
    pub fn new(client: Object) -> Self {
        Self(client)
    }
}

#[async_trait]
impl PostgresClient for DefaultPostgresClient {
    fn into_client(self: Box<Self>) -> Object {
        self.0
    }

    async fn transaction(&mut self) -> PostgresResult<Box<dyn PostgresTransaction + '_>> {
        trace!("opening transaction");
        let tx = self.0.transaction().await?;
        Ok(Box::new(DefaultPostgresTransaction(tx)))
    }

    fn upcast(&self) -> &dyn ToPostgresClient {
        self
    }
}

impl ToPostgresClient for DefaultPostgresClient {
    fn to_client(&self) -> &Client {
        self.0.deref().deref()
    }
}

// DefaultPostgresPool

/// Default implementation of [`PostgresPool`](trait.PostgresPool.html).
///
/// **This is supported on `feature=postgres` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
pub struct DefaultPostgresPool(Pool);

impl DefaultPostgresPool {
    /// Create a new `DefaultPostgresPool`.
    pub fn new(pool: Pool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl PostgresPool for DefaultPostgresPool {
    async fn get(&self) -> PostgresResult<Box<dyn PostgresClient>> {
        trace!("getting client from pool");
        let client = self.0.get().await?;
        Ok(Box::new(DefaultPostgresClient(client)))
    }
}

// DefaultPostgresTransaction

/// Default implementation of [`PostgresTransaction`](trait.PostgresTransaction.html).
///
/// **This is supported on `feature=postgres` only.**
pub struct DefaultPostgresTransaction<'a>(Transaction<'a>);

impl<'a> DefaultPostgresTransaction<'a> {
    /// Create a new `DefaultPostgresTransaction`.
    pub fn new(transaction: Transaction<'a>) -> Self {
        Self(transaction)
    }
}

#[async_trait]
impl<'a> PostgresTransaction<'a> for DefaultPostgresTransaction<'a> {
    fn to_transaction(&self) -> &Transaction<'a> {
        &self.0
    }

    async fn commit(self: Box<Self>) -> PostgresResult<()> {
        trace!("committing transaction");
        self.0.commit().await?;
        Ok(())
    }

    fn into_transaction(self: Box<Self>) -> Transaction<'a> {
        self.0
    }

    async fn rollback(self: Box<Self>) -> PostgresResult<()> {
        trace!("rolling back transaction");
        self.0.rollback().await?;
        Ok(())
    }

    fn upcast(&self) -> &dyn ToPostgresClient {
        self
    }
}

impl ToPostgresClient for DefaultPostgresTransaction<'_> {
    fn to_client(&self) -> &Client {
        self.0.client()
    }
}

// MockPostgresClient

/// Mock implementation of [`PostgresClient`](trait.PostgresClient.html).
///
/// **This is supported on `feature=postgres,mock` only.**
///
/// Note this mock is not `mockall` compatible because of lifetime issues.
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
#[cfg(feature = "mock")]
#[derive(Default)]
pub struct MockPostgresClient {
    /// Mock implementation of [`PostgresClient::transaction`](trait.PostgresClient.html#method.transaction).
    pub transaction: crate::Mock<MockPostgresTransaction>,
}

#[cfg(feature = "mock")]
#[async_trait]
impl PostgresClient for MockPostgresClient {
    /// **This method is unimplemented.**
    fn into_client(self: Box<Self>) -> Object {
        unimplemented!()
    }

    async fn transaction(&mut self) -> PostgresResult<Box<dyn PostgresTransaction + '_>> {
        Ok(Box::new(self.transaction.call()))
    }

    fn upcast(&self) -> &dyn ToPostgresClient {
        self
    }
}

#[cfg(feature = "mock")]
impl ToPostgresClient for MockPostgresClient {
    /// **This method is unimplemented.**
    fn to_client(&self) -> &Client {
        unimplemented!()
    }
}

// MockPostgresPool

#[cfg(feature = "mock")]
mockall::mock! {
    /// Mock implementation of [`PostgresPool`](trait.PostgresPool.html).
    ///
    /// **This is supported on `feature=postgres,mock` only.**
    ///
    /// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
    pub PostgresPool {}

    #[async_trait]
    impl PostgresPool for PostgresPool {
        async fn get(&self) -> PostgresResult<Box<dyn PostgresClient>>;
    }
}

// MockPostgresTransaction

/// Mock implementation of [`PostgresTransaction`](trait.PostgresTransaction.html).
///
/// **This is supported on `feature=postgres,mock` only.**
///
/// Note this mock is not `mockall` compatible because of lifetime issues.
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
#[cfg(feature = "mock")]
#[derive(Default)]
pub struct MockPostgresTransaction {
    /// Mock implementation of [`PostgresTransaction::commit`](trait.PostgresTransaction.html#method.commit).
    pub commit: crate::Mock<PostgresResult<()>>,
    /// Mock implementation of [`PostgresTransaction::rollback`](trait.PostgresTransaction.html#method.rollback).
    pub rollback: crate::Mock<PostgresResult<()>>,
}

#[cfg(feature = "mock")]
#[async_trait]
impl<'a> PostgresTransaction<'a> for MockPostgresTransaction {
    /// **This method is unimplemented.**
    fn to_transaction(&self) -> &Transaction<'a> {
        unimplemented!()
    }

    async fn commit(self: Box<Self>) -> PostgresResult<()> {
        self.commit.call()
    }

    /// **This method is unimplemented.**
    fn into_transaction(self: Box<Self>) -> Transaction<'a> {
        unimplemented!()
    }

    async fn rollback(self: Box<Self>) -> PostgresResult<()> {
        self.rollback.call()
    }

    fn upcast(&self) -> &dyn ToPostgresClient {
        self
    }
}

#[cfg(feature = "mock")]
impl ToPostgresClient for MockPostgresTransaction {
    /// **This method is unimplemented.**
    fn to_client(&self) -> &Client {
        unimplemented!()
    }
}

// transactional

/// Runs a function in a transaction.
///
/// **This is supported on `feature=postgres` only.**
///
/// [Example](https://github.com/leroyguillaume/mockable/tree/main/examples/postgres.rs).
pub async fn transactional<
    'a,
    T,
    E,
    F: for<'b> Fn(&'b dyn PostgresTransaction) -> Pin<Box<dyn Future<Output = Result<T, E>> + 'b>>,
>(
    client: &'a mut dyn PostgresClient,
    f: F,
) -> PostgresResult<Result<T, E>> {
    let tx = client.transaction().await?;
    match f(tx.as_ref()).await {
        Ok(val) => {
            tx.commit().await?;
            Ok(Ok(val))
        }
        Err(err) => {
            tx.rollback().await?;
            Ok(Err(err))
        }
    }
}
