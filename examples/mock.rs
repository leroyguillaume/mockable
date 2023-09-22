use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use mockable::{DefaultFileSystem, FileSystem};

struct Cache(Box<dyn FileSystem>);

impl Cache {
    fn new() -> Self {
        Self(Box::new(DefaultFileSystem))
    }

    fn load_all(&self) -> HashMap<PathBuf, String> {
        let root = Path::new("/tmp/mockable-test");
        let entries = self.0.read_dir(root).expect("reading diectory failed");
        let mut cache = HashMap::new();
        for entry in entries {
            let entry = entry.expect("reading entry failed");
            let path = entry.path();
            let content = self.0.read_to_string(&path).expect("reading cache failed");
            cache.insert(path, content);
        }
        cache
    }

    fn save(&self, filename: &str, content: &str) {
        let root = Path::new("/tmp/mockable-test");
        let path = root.join(filename);
        self.0
            .create_dir_all(root)
            .expect("creating directory failed");
        self.0
            .write(&path, content.as_bytes())
            .expect("writing cache failed")
    }
}

fn main() {
    let cache = Cache::new();
    cache.save("test", "Hello, world!");
    let entries = cache.load_all();
    println!("{entries:?}");
}

#[cfg(test)]
mod test {
    use std::io::Result;

    use mockable::{DirEntry, Mock, MockDirEntry, MockFileSystem, VecReadDir};
    use mockall::predicate::eq;

    use super::*;

    struct Functions {
        read_to_string: Mock<Result<String>>,
    }

    struct Params {
        content: &'static str,
        filename: &'static str,
        root: &'static Path,
    }

    fn run(params: Params, fns: Functions) -> HashMap<PathBuf, String> {
        let mut fs = MockFileSystem::new();
        fs.expect_create_dir_all()
            .with(eq(params.root))
            .returning(|_| Ok(()));
        fs.expect_write()
            .with(
                eq(params.root.join(params.filename)),
                eq(params.content.as_bytes()),
            )
            .returning(|_, _| Ok(()));
        fs.expect_read_dir()
            .with(eq(params.root))
            .returning(move |_| {
                let mut entry = MockDirEntry::new();
                entry
                    .expect_path()
                    .return_const(params.root.join(params.filename));
                let entry: Box<dyn DirEntry> = Box::new(entry);
                Ok(Box::new(VecReadDir::from(vec![Ok(entry)])))
            });
        fs.expect_read_to_string()
            .with(eq(params.root.join(params.filename)))
            .returning({
                let mock = fns.read_to_string.clone();
                move |_| mock.call()
            });
        let cache = Cache(Box::new(fs));
        cache.save(params.filename, params.content);
        cache.load_all()
    }

    #[test]
    fn test() {
        let params = Params {
            content: "Hello world!",
            filename: "test",
            root: Path::new("/tmp/mockable-test"),
        };
        let expected =
            HashMap::from_iter([(params.root.join(params.filename), params.content.into())]);
        let fns = Functions {
            read_to_string: Mock::once(|| Ok(params.content.into())),
        };
        let entries = run(params, fns);
        assert_eq!(entries, expected);
    }
}
