use std::{
    ffi::OsString,
    io::Result,
    path::{Path, PathBuf},
    time::SystemTime,
};

use tracing::trace;

// DirEntry

/// A trait for directory entry.
pub trait DirEntry: Send + Sync {
    /// See [`std::fs::DirEntry::file_name`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.path) for more details.
    fn file_name(&self) -> OsString;

    /// Converts this trait object into a [`std::fs::DirEntry`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html) instance.
    fn into_dir_entry(self: Box<Self>) -> std::fs::DirEntry;

    /// See [`std::fs::DirEntry::metadata`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.metadata) for more details.
    fn metadata(&self) -> Result<Box<dyn Metadata>>;

    /// See [`std::fs::DirEntry::path`](https://doc.rust-lang.org/stable/std/fs/struct.DirEntry.html#method.path) for more details.
    fn path(&self) -> PathBuf;
}

// FileSystem

/// A trait for file system operations.
///
/// # Examples
///
/// ```
/// use std::{io::Result, path::Path};
///
/// use mockall::predicate::eq;
/// use mockable::{DefaultFileSystem, FileSystem, Metadata, MockFileSystem, MockMetadata};
///
/// fn get_metadata(path: &Path, fs: &dyn FileSystem) -> Result<Box<dyn Metadata>> {
///     fs.metadata(path)
/// }
///
/// // Default
/// let metadata = get_metadata(Path::new("/"), &DefaultFileSystem).unwrap();
/// assert!(metadata.is_dir());
///
/// // Mock
/// let mut fs = MockFileSystem::new();
/// fs
///     .expect_metadata()
///     .with(eq(Path::new("/")))
///     .returning(|_| {
///         let mut metadata = MockMetadata::new();
///         metadata
///             .expect_is_dir()
///             .returning(|| true);
///         Ok(Box::new(metadata))
///     });
/// let metadata = get_metadata(Path::new("/"), &fs).unwrap();
/// assert!(metadata.is_dir());
/// ```
pub trait FileSystem: Send + Sync {
    /// See [`std::fs::copy`](https://doc.rust-lang.org/stable/std/fs/fn.copy.html) for more details.
    fn copy(&self, from: &Path, to: &Path) -> Result<u64>;

    /// See [`std::fs::create_dir`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir.html) for more details.
    fn create_dir(&self, path: &Path) -> Result<()>;

    /// See [`std::fs::create_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.create_dir_all.html) for more details.
    fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// See [`std::fs::hard_link`](https://doc.rust-lang.org/stable/std/fs/fn.hard_link.html) for more details.
    fn hard_link(&self, original: &Path, link: &Path) -> Result<()>;

    /// See [`std::fs::metadata`](https://doc.rust-lang.org/stable/std/fs/fn.metadata.html) for more details.
    fn metadata(&self, path: &Path) -> Result<Box<dyn Metadata>>;

    /// See [`std::fs::read`](https://doc.rust-lang.org/stable/std/fs/fn.read.html) for more details.
    fn read(&self, path: &Path) -> Result<Vec<u8>>;

    /// See [`std::fs::read_dir`](https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html) for more details.
    fn read_dir(&self, path: &Path) -> Result<Box<dyn ReadDir>>;

    /// See [`std::fs::read_link`](https://doc.rust-lang.org/stable/std/fs/fn.read_link.html) for more details.
    fn read_link(&self, path: &Path) -> Result<PathBuf>;

    /// See [`std::fs::read_to_string`](https://doc.rust-lang.org/stable/std/fs/fn.read_to_string.html) for more details.
    fn read_to_string(&self, path: &Path) -> Result<String>;

    /// See [`std::fs::remove_dir`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir.html) for more details.
    fn remove_dir(&self, path: &Path) -> Result<()>;

    /// See [`std::fs::remove_dir_all`](https://doc.rust-lang.org/stable/std/fs/fn.remove_dir_all.html) for more details.
    fn remove_dir_all(&self, path: &Path) -> Result<()>;

    /// See [`std::fs::remove_file`](https://doc.rust-lang.org/stable/std/fs/fn.remove_file.html) for more details.
    fn remove_file(&self, path: &Path) -> Result<()>;

    /// See [`std::fs::rename`](https://doc.rust-lang.org/stable/std/fs/fn.rename.html) for more details.
    fn rename(&self, from: &Path, to: &Path) -> Result<()>;

    /// See [`std::fs::set_permissions`](https://doc.rust-lang.org/stable/std/fs/fn.set_permissions.html) for more details.
    fn set_permissions(&self, path: &Path, perm: Box<dyn Permissions>) -> Result<()>;

    /// See [`std::os::unix::fs::symlink`](https://doc.rust-lang.org/stable/std/os/unix/fs/fn.symlink.html) for more details.
    ///
    /// **This is supported on `os=unix` only.**
    #[cfg(unix)]
    fn symlink(&self, original: &Path, link: &Path) -> Result<()>;

    /// See [`std::fs::symlink_metadata`](https://doc.rust-lang.org/stable/std/fs/fn.symlink_metadata.html) for more details.
    fn symlink_metadata(&self, path: &Path) -> Result<Box<dyn Metadata>>;

    /// See [`std::fs::write`](https://doc.rust-lang.org/stable/std/fs/fn.write.html) for more details.
    fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;
}

// Metadata

/// A trait for file metadata.
#[allow(clippy::len_without_is_empty)]
pub trait Metadata: Send + Sync {
    /// See [`std::fs::Metadata::accessed`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.accessed) for more details.
    fn accessed(&self) -> Result<SystemTime>;

    /// See [`std::fs::Metadata::created`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.created) for more details.
    fn created(&self) -> Result<SystemTime>;

    /// Converts this trait object into a [`std::fs::Metadata`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html) instance.
    fn into_metadata(self: Box<Self>) -> std::fs::Metadata;

    /// See [`std::fs::Metadata::is_dir`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.is_dir) for more details.
    fn is_dir(&self) -> bool;

    /// See [`std::fs::Metadata::is_file`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.is_file) for more details.
    fn is_file(&self) -> bool;

    /// See [`std::fs::Metadata::is_symlink`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.is_symlink) for more details.
    fn is_symlink(&self) -> bool;

    /// See [`std::fs::Metadata::len`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.len) for more details.
    fn len(&self) -> u64;

    /// See [`std::fs::Metadata::modified`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.modified) for more details.
    fn modified(&self) -> Result<SystemTime>;

    /// See [`std::fs::Metadata::permissions`](https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.permissions) for more details.
    fn permissions(&self) -> Box<dyn Permissions>;
}

// Permissions

/// A trait for file permissions.
pub trait Permissions: Send + Sync {
    /// Converts this trait object into a [`std::fs::Permissions`](https://doc.rust-lang.org/stable/std/fs/struct.Permissions.html) instance.
    fn into_permissions(self: Box<Self>) -> std::fs::Permissions;

    /// See [`std::os::unix::fs::PermissionsExt::mode`](https://doc.rust-lang.org/stable/std/os/unix/fs/trait.PermissionsExt.html#tymethod.mode) for more details.
    ///
    /// **This is supported on `os=unix` only.**
    #[cfg(unix)]
    fn mode(&self) -> u32;

    /// See [`std::fs::Permissions::readonly`](https://doc.rust-lang.org/stable/std/fs/struct.Permissions.html#method.readonly) for more details.
    fn readonly(&self) -> bool;

    /// See [`std::os::unix::fs::PermissionsExt::set_mode`](https://doc.rust-lang.org/stable/std/os/unix/fs/trait.PermissionsExt.html#tymethod.set_mode) for more details.
    ///
    /// **This is supported on `os=unix` only.**
    #[cfg(unix)]
    fn set_mode(&mut self, mode: u32);

    /// See [`std::fs::Permissions::set_readonly`](https://doc.rust-lang.org/stable/std/fs/struct.Permissions.html#method.set_readonly) for more details.
    fn set_readonly(&mut self, readonly: bool);
}

// ReadDir

/// A trait for directory iterator.
pub trait ReadDir: Iterator<Item = Result<Box<dyn DirEntry>>> + Send + Sync {}

// DefaultDirEntry

/// Default implementation of [`DirEntry`](trait.DirEntry.html).
pub struct DefaultDirEntry(std::fs::DirEntry);

impl From<std::fs::DirEntry> for DefaultDirEntry {
    fn from(entry: std::fs::DirEntry) -> Self {
        Self(entry)
    }
}

impl DirEntry for DefaultDirEntry {
    fn file_name(&self) -> OsString {
        self.0.file_name()
    }

    fn into_dir_entry(self: Box<Self>) -> std::fs::DirEntry {
        self.0
    }

    fn metadata(&self) -> Result<Box<dyn Metadata>> {
        let metadata = self.0.metadata()?;
        Ok(Box::new(DefaultMetadata(metadata)))
    }

    fn path(&self) -> PathBuf {
        self.0.path()
    }
}

// DefaultFileSystem

/// Default implementation of [`FileSystem`](trait.FileSystem.html).
pub struct DefaultFileSystem;

impl FileSystem for DefaultFileSystem {
    fn copy(&self, from: &Path, to: &Path) -> Result<u64> {
        trace!(from = %from.display(), to = %to.display(), "copying file");
        std::fs::copy(from, to)
    }

    fn create_dir(&self, path: &Path) -> Result<()> {
        trace!(path = %path.display(), "creating directory");
        std::fs::create_dir(path)
    }

    fn create_dir_all(&self, path: &Path) -> Result<()> {
        trace!(path = %path.display(), "creating directory recursively");
        std::fs::create_dir_all(path)
    }

    fn hard_link(&self, original: &Path, link: &Path) -> Result<()> {
        trace!(original = %original.display(), link = %link.display(), "creating hard link");
        std::fs::hard_link(original, link)
    }

    fn metadata(&self, path: &Path) -> Result<Box<dyn Metadata>> {
        trace!(path = %path.display(), "getting metadata");
        let metadata = std::fs::metadata(path)?;
        Ok(Box::new(DefaultMetadata(metadata)))
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        trace!(path = %path.display(), "reading file");
        std::fs::read(path)
    }

    fn read_dir(&self, path: &Path) -> Result<Box<dyn ReadDir>> {
        trace!(path = %path.display(), "reading directory");
        let dir = std::fs::read_dir(path)?;
        Ok(Box::new(DefaultReadDir(dir)))
    }

    fn read_link(&self, path: &Path) -> Result<PathBuf> {
        trace!(path = %path.display(), "reading link");
        std::fs::read_link(path)
    }

    fn read_to_string(&self, path: &Path) -> Result<String> {
        trace!(path = %path.display(), "reading file");
        std::fs::read_to_string(path)
    }

    fn remove_dir(&self, path: &Path) -> Result<()> {
        trace!(path = %path.display(), "removing directory");
        std::fs::remove_dir(path)
    }

    fn remove_dir_all(&self, path: &Path) -> Result<()> {
        trace!(path = %path.display(), "removing directory recursively");
        std::fs::remove_dir_all(path)
    }

    fn remove_file(&self, path: &Path) -> Result<()> {
        trace!(path = %path.display(), "removing file");
        std::fs::remove_file(path)
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<()> {
        trace!(from = %from.display(), to = %to.display(), "renaming file");
        std::fs::rename(from, to)
    }

    fn set_permissions(&self, path: &Path, perms: Box<dyn Permissions>) -> Result<()> {
        trace!(path = %path.display(), "setting permissions");
        std::fs::set_permissions(path, perms.into_permissions())
    }

    #[cfg(unix)]
    fn symlink(&self, original: &Path, link: &Path) -> Result<()> {
        trace!(original = %original.display(), link = %link.display(), "creating symlink");
        std::os::unix::fs::symlink(original, link)
    }

    fn symlink_metadata(&self, path: &Path) -> Result<Box<dyn Metadata>> {
        trace!(path = %path.display(), "getting symlink metadata");
        let metadata = std::fs::symlink_metadata(path)?;
        Ok(Box::new(DefaultMetadata(metadata)))
    }

    fn write(&self, path: &Path, content: &[u8]) -> Result<()> {
        trace!(path = %path.display(), "writing into file");
        std::fs::write(path, content)
    }
}

// DefaultMetadata

/// Default implementation of [`Metadata`](trait.Metadata.html).
pub struct DefaultMetadata(std::fs::Metadata);

impl From<std::fs::Metadata> for DefaultMetadata {
    fn from(metadata: std::fs::Metadata) -> Self {
        Self(metadata)
    }
}

impl Metadata for DefaultMetadata {
    fn accessed(&self) -> Result<SystemTime> {
        self.0.accessed()
    }

    fn created(&self) -> Result<SystemTime> {
        self.0.created()
    }

    fn into_metadata(self: Box<Self>) -> std::fs::Metadata {
        self.0
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn is_symlink(&self) -> bool {
        self.0.is_symlink()
    }

    fn len(&self) -> u64 {
        self.0.len()
    }

    fn modified(&self) -> Result<SystemTime> {
        self.0.modified()
    }

    fn permissions(&self) -> Box<dyn Permissions> {
        Box::new(DefaultPermissions(self.0.permissions()))
    }
}

// DefaultPermissions

/// Default implementation of [`Permissions`](trait.Permissions.html).
pub struct DefaultPermissions(std::fs::Permissions);

impl From<std::fs::Permissions> for DefaultPermissions {
    fn from(perms: std::fs::Permissions) -> Self {
        Self(perms)
    }
}

impl Permissions for DefaultPermissions {
    fn into_permissions(self: Box<Self>) -> std::fs::Permissions {
        self.0
    }

    #[cfg(unix)]
    fn mode(&self) -> u32 {
        use std::os::unix::prelude::PermissionsExt;

        self.0.mode()
    }

    fn readonly(&self) -> bool {
        self.0.readonly()
    }

    #[cfg(unix)]
    fn set_mode(&mut self, mode: u32) {
        use std::os::unix::prelude::PermissionsExt;

        self.0.set_mode(mode)
    }

    fn set_readonly(&mut self, readonly: bool) {
        self.0.set_readonly(readonly)
    }
}

// DefaultReadDir

/// Default implementation of [`ReadDir`](trait.ReadDir.html).
pub struct DefaultReadDir(std::fs::ReadDir);

impl From<std::fs::ReadDir> for DefaultReadDir {
    fn from(dir: std::fs::ReadDir) -> Self {
        Self(dir)
    }
}

impl Iterator for DefaultReadDir {
    type Item = Result<Box<dyn DirEntry>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|entry| entry.map(|entry| Box::new(DefaultDirEntry(entry)) as Box<dyn DirEntry>))
    }
}

impl ReadDir for DefaultReadDir {}

// VecReadDir

/// A [`ReadDir`](trait.ReadDir.html) implementation that reads from a vector.
pub struct VecReadDir(Vec<Result<Box<dyn DirEntry>>>);

impl From<Vec<Result<Box<dyn DirEntry>>>> for VecReadDir {
    fn from(entries: Vec<Result<Box<dyn DirEntry>>>) -> Self {
        Self(entries)
    }
}

impl IntoIterator for VecReadDir {
    type Item = Result<Box<dyn DirEntry>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// MockDirEntry

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`DirEntry`](trait.DirEntry.html).
    ///
    /// **This is supported on `feature=mock` only.**
    pub DirEntry {}

    impl DirEntry for DirEntry {
        fn file_name(&self) -> OsString;

        fn into_dir_entry(self: Box<Self>) -> std::fs::DirEntry;

        fn metadata(&self) -> Result<Box<dyn Metadata>>;

        fn path(&self) -> PathBuf;
    }
}

// MockFileSystem

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`FileSystem`](trait.FileSystem.html).
    ///
    /// **This is supported on `feature=mock` only.**
    pub FileSystem {}

    impl FileSystem for FileSystem {
        fn copy(&self, from: &Path, to: &Path) -> Result<u64>;

        fn create_dir(&self, path: &Path) -> Result<()>;

        fn create_dir_all(&self, path: &Path) -> Result<()>;

        fn hard_link(&self, original: &Path, link: &Path) -> Result<()>;

        fn metadata(&self, path: &Path) -> Result<Box<dyn Metadata>>;

        fn read(&self, path: &Path) -> Result<Vec<u8>>;

        fn read_dir(&self, path: &Path) -> Result<Box<dyn ReadDir>>;

        fn read_link(&self, path: &Path) -> Result<PathBuf>;

        fn read_to_string(&self, path: &Path) -> Result<String>;

        fn remove_dir(&self, path: &Path) -> Result<()>;

        fn remove_dir_all(&self, path: &Path) -> Result<()>;

        fn remove_file(&self, path: &Path) -> Result<()>;

        fn rename(&self, from: &Path, to: &Path) -> Result<()>;

        fn set_permissions(&self, path: &Path, perm: Box<dyn Permissions>) -> Result<()>;

        #[cfg(unix)]
        fn symlink(&self, original: &Path, link: &Path) -> Result<()>;

        fn symlink_metadata(&self, path: &Path) -> Result<Box<dyn Metadata>>;

        fn write(&self, path: &Path, contents: &[u8]) -> Result<()>;
    }
}

// MockMetadata

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`Metadata`](trait.Metadata.html).
    ///
    /// **This is supported on `feature=mock` only.**
    pub Metadata {}

    impl Metadata for Metadata {
        fn accessed(&self) -> Result<SystemTime>;

        fn created(&self) -> Result<SystemTime>;

        fn into_metadata(self: Box<Self>) -> std::fs::Metadata;

        fn is_dir(&self) -> bool;

        fn is_file(&self) -> bool;

        fn is_symlink(&self) -> bool;

        fn len(&self) -> u64;

        fn modified(&self) -> Result<SystemTime>;

        fn permissions(&self) -> Box<dyn Permissions>;
    }
}

// MockPermissions

#[cfg(feature = "mock")]
mockall::mock! {
    /// `mockall` implementation of [`Permissions`](trait.Permissions.html).
    ///
    /// **This is supported on `feature=mock` only.**
    pub Permissions {}

    impl Permissions for Permissions {
        fn into_permissions(self: Box<Self>) -> std::fs::Permissions;

        #[cfg(unix)]
        fn mode(&self) -> u32;

        fn readonly(&self) -> bool;

        #[cfg(unix)]
        fn set_mode(&mut self, mode: u32);

        fn set_readonly(&mut self, readonly: bool);
    }
}
