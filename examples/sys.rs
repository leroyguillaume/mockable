use mockable::{DefaultSystem, System};

struct OAuth(Box<dyn System>);

impl OAuth {
    fn new() -> Self {
        Self(Box::new(DefaultSystem))
    }

    fn authenticate(&self, redirect_uri: &str) {
        let url = format!("https://example.com/auth?redirect_uri={redirect_uri}");
        self.0.open_url(&url).expect("failed to open browser");
    }
}

fn main() {
    let oauth = OAuth::new();
    oauth.authenticate("https://example.com/callback");
}

#[cfg(test)]
mod test {
    use mockable::MockSystem;
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test() {
        let redirect_uri = "https://example.com/callback";
        let url = format!("https://example.com/auth?redirect_uri={redirect_uri}");
        let mut system = MockSystem::new();
        system.expect_open_url().with(eq(url)).returning(|_| Ok(()));
        let oauth = OAuth(Box::new(system));
        oauth.authenticate(redirect_uri);
    }
}
