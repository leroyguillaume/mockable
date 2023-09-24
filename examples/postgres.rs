use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::{
    tokio_postgres::{Client, NoTls},
    Config,
};
use mockable::{
    transactional, DefaultEnv, DefaultPostgresPool, Env, PostgresError, PostgresPool,
    PostgresResult, ToPostgresClient,
};
use mockall::automock;

#[derive(Clone, Debug, Eq, PartialEq)]
struct User {
    id: i32,
    email: String,
}

trait RepositoryProvider {
    fn user<'a>(&self, client: &'a dyn ToPostgresClient) -> Box<dyn UserRepository + 'a>;
}

#[automock]
#[async_trait]
trait UserRepository {
    async fn create(&self, email: String) -> PostgresResult<User>;
}

struct DefaultRepositoryProvider;

impl RepositoryProvider for DefaultRepositoryProvider {
    fn user<'a>(&self, client: &'a dyn ToPostgresClient) -> Box<dyn UserRepository + 'a> {
        Box::new(DefaultUserRepository(client.to_client()))
    }
}

struct DefaultUserRepository<'a>(&'a Client);

#[async_trait]
impl<'a> UserRepository for DefaultUserRepository<'a> {
    async fn create(&self, email: String) -> PostgresResult<User> {
        let client = self.0;
        let row = client
            .query_one(
                "INSERT INTO \"user\" (email) VALUES ($1) RETURNING id, email",
                &[&email],
            )
            .await?;
        Ok(User {
            id: row.get(0),
            email: row.get(1),
        })
    }
}

#[tokio::main]
async fn main() {
    let env = DefaultEnv::default();
    let cfg = Config {
        dbname: env.string("DB_NAME"),
        host: env.string("DB_HOST"),
        password: env.string("DB_PASSWORD"),
        port: env
            .u16("DB_PORT")
            .map(|port| port.expect("parsing DB_PORT failed")),
        user: env.string("DB_USER"),
        ..Default::default()
    };
    let pool = cfg.create_pool(None, NoTls).expect("creating pool failed");
    let pool = DefaultPostgresPool::new(pool);
    let provider = Arc::new(DefaultRepositoryProvider);
    let mut client = pool.get().await.expect("getting client from pool failed");
    let user = transactional(client.as_mut(), move |tx| {
        let provider = provider.clone();
        Box::pin(async move {
            let repo = provider.user(tx.upcast());
            let user = repo.create("user@test".into()).await?;
            Ok::<User, PostgresError>(user)
        })
    })
    .await
    .expect("database client failed")
    .expect("creating user failed");
    println!("{user:?}");
}

#[cfg(test)]
mod test {
    use mockable::{Mock, MockPostgresClient, MockPostgresPool, MockPostgresTransaction};
    use mockall::predicate::eq;

    use super::*;

    struct MockRepositoryProvider {
        user: Mock<MockUserRepository>,
    }

    impl RepositoryProvider for MockRepositoryProvider {
        fn user<'a>(&self, _: &'a dyn ToPostgresClient) -> Box<dyn UserRepository + 'a> {
            Box::new(self.user.call())
        }
    }

    #[tokio::test]
    async fn test() {
        let expected = User {
            id: 1,
            email: "user@test".into(),
        };
        let mut pool = MockPostgresPool::new();
        pool.expect_get().returning(|| {
            let client = MockPostgresClient {
                transaction: Mock::once(|| MockPostgresTransaction {
                    commit: Mock::once(|| Ok(())),
                    ..Default::default()
                }),
            };
            Ok(Box::new(client))
        });
        let provider = Arc::new(MockRepositoryProvider {
            user: Mock::once({
                let expected = expected.clone();
                move || {
                    let mut repo = MockUserRepository::new();
                    repo.expect_create()
                        .with(eq(expected.email.clone()))
                        .returning({
                            let expected = expected.clone();
                            move |_| Ok(expected.clone())
                        });
                    repo
                }
            }),
        });
        let mut client = pool.get().await.expect("getting client from pool failed");
        let user = transactional(client.as_mut(), {
            let expected = expected.clone();
            move |tx| {
                let expected = expected.clone();
                let provider = provider.clone();
                Box::pin(async move {
                    let repo = provider.user(tx.upcast());
                    let user = repo.create(expected.email.clone()).await?;
                    Ok::<User, PostgresError>(user)
                })
            }
        })
        .await
        .expect("database client failed")
        .expect("creating user failed");
        assert_eq!(user, expected);
    }
}
