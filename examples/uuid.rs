use mockable::{DefaultUuidGenerator, UuidGenerator};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
struct User {
    id: Uuid,
}

fn create(gen: &dyn UuidGenerator) -> User {
    User {
        id: gen.generate_v4(),
    }
}

fn main() {
    let user = create(&DefaultUuidGenerator);
    println!("{user:?}");
}

#[cfg(test)]
mod test {
    use mockable::MockUuidGenerator;

    use super::*;

    #[test]
    fn test() {
        let expected = Uuid::new_v4();
        let mut gen = MockUuidGenerator::new();
        gen.expect_generate_v4().returning(move || expected);
        let user = create(&gen);
        assert_eq!(user.id, expected);
    }
}
