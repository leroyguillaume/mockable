use mockable::{DefaultEnv, Env};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    secret: String,
}

struct ConfigLoader(Box<dyn Env>);

impl ConfigLoader {
    fn new() -> Self {
        Self(Box::new(DefaultEnv::new()))
    }

    fn load(&self) -> Config {
        Config {
            secret: self.0.string("SECRET").expect("SECRET is not set"),
        }
    }
}

fn main() {
    let loader = ConfigLoader::new();
    let config = loader.load();
    println!("{config:?}");
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
        let loader = ConfigLoader(Box::new(env));
        let cfg = loader.load();
        assert_eq!(cfg, expected);
    }
}
