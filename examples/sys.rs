use mockable::{DefaultSystem, System};

fn open_browser(url: &str, sys: &dyn System) {
    sys.open_url(url).expect("failed to open browser");
}

fn main() {
    open_browser("https://google.com", &DefaultSystem)
}

#[cfg(test)]
mod test {
    use mockable::MockSystem;
    use mockall::predicate::eq;

    use super::*;

    #[test]
    fn test() {
        let url = "https://google.com";
        let mut sys = MockSystem::new();
        sys.expect_open_url().with(eq(url)).returning(|_| Ok(()));
        open_browser(url, &sys)
    }
}
