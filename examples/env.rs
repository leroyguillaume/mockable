use mockable::{DefaultEnv, Env};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    secret: String,
}

fn load(env: &dyn Env) -> Config {
    Config {
        secret: env.string("SECRET").expect("SECRET is not set"),
    }
}

fn main() {
    let cfg = load(&DefaultEnv);
    println!("{cfg:?}");
}

#[cfg(test)]
mod test {
    use mockable::MockEnv;
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test() {
        let expected = Config {
            secret: "secret".into(),
        };
        let mut env = MockEnv::new();
        env.expect_string().with(eq("SECRET")).returning({
            let expected = expected.clone();
            move |_| Some(expected.secret.clone())
        });
        let cfg = load(&env);
        assert_eq!(cfg, expected);
    }
}
