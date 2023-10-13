use chrono::{DateTime, Utc};
use mockable::{Clock, DefaultClock};

#[derive(Debug, Eq, PartialEq)]
#[allow(dead_code)]
struct User {
    creation: DateTime<Utc>,
    name: String,
}

fn create_user(name: String, clock: &dyn Clock) -> User {
    User {
        creation: clock.utc(),
        name,
    }
}

fn main() {
    let user = create_user("Alice".into(), &DefaultClock);
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
        let expected_user = User {
            creation,
            name: "Alice".into(),
        };
        let user = create_user(expected_user.name.clone(), &clock);
        assert_eq!(user, expected_user);
    }
}
