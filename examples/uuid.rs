use mockable::{DefaultUuidGenerator, UuidGenerator};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
struct User {
    id: Uuid,
}

struct UserRegistry(Box<dyn UuidGenerator>);

impl UserRegistry {
    fn new() -> Self {
        Self(Box::new(DefaultUuidGenerator))
    }

    fn create(&self) -> User {
        User {
            id: self.0.generate_v4(),
        }
    }
}

fn main() {
    let registry = UserRegistry::new();
    let user = registry.create();
    println!("{user:?}");
}

#[cfg(test)]
mod test {
    use mockable::MockUuidGenerator;

    use super::*;

    #[test]
    fn test() {
        let expected = Uuid::new_v4();
        let mut generator = MockUuidGenerator::new();
        generator.expect_generate_v4().returning(move || expected);
        let registry = UserRegistry(Box::new(generator));
        let user = registry.create();
        assert_eq!(user.id, expected);
    }
}
