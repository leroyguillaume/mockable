use mockable::{DefaultEnv, Env};

fn main() {
    let env = DefaultEnv;
    let val_1 = env.string("SECRET_1");
    let val_2 = env.string("SECRET_2");
    println!("{val_1:?} {val_2:?}");
}

#[cfg(test)]
mod test {
    use mockable::{Mock, MockEnv};

    use super::*;

    #[test]
    fn test() {
        let mock = Mock::with(vec![
            Box::new(|key: String| {
                assert_eq!(key, "SECRET_1");
                Some("val_1".into())
            }),
            Box::new(|key: String| {
                assert_eq!(key, "SECRET_2");
                Some("val_2".into())
            }),
        ]);
        let mut env = MockEnv::new();
        env.expect_string()
            .returning(move |key| mock.call_with_args(key.into()));
        let val_1 = env.string("SECRET_1");
        let val_2 = env.string("SECRET_2");
        assert_eq!(val_1, Some("val_1".into()));
        assert_eq!(val_2, Some("val_2".into()));
    }
}
