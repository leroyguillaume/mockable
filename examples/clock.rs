use chrono::{DateTime, Utc};
use mockable::{Clock, DefaultClock};

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    creation: DateTime<Utc>,
    name: String,
}

struct UserRegistry(Box<dyn Clock>);

impl UserRegistry {
    fn new() -> Self {
        Self(Box::new(DefaultClock))
    }

    fn create(&self, name: String) -> User {
        User {
            creation: self.0.utc(),
            name,
        }
    }
}

fn main() {
    let registry = UserRegistry::new();
    let user = registry.create("Alice".into());
    println!("{user:?}");
}

#[cfg(test)]
mod test {
    use mockable::MockClock;

    use super::*;

    #[test]
    fn test() {
        let creation = Utc::now();
        let mut clock = MockClock::new();
        clock.expect_utc().return_const(creation);
        let registry = UserRegistry(Box::new(clock));
        let user = registry.create("Alice".into());
        assert_eq!(user.creation, creation);
    }
}
