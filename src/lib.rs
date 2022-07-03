//! A library for converting file paths to and from "slash paths".
//!
//! A "slash path" is a path whose components are always separated by `/` and never `\`.
//!
//! On Unix-like OS, the path separator is `/`. So any conversion is not necessary.
//! But on Windows, the file path separator is `\`, and needs to be replaced with `/` for converting
//! the paths to "slash paths". Of course, `\`s used for escaping characters should not be replaced.
//!
//! For example, a file path `foo\bar\piyo.txt` can be converted to/from a slash path `foo/bar/piyo.txt`.
//!
//! This package was inspired by Go's [`path/filepath.FromSlash`](https://golang.org/pkg/path/filepath/#FromSlash)
//! and [`path/filepath.ToSlash`](https://golang.org/pkg/path/filepath/#ToSlash).
//!
//! ```rust
//! use std::path::{Path, PathBuf};
//! use std::borrow::Cow;
//!
//! // Trait for extending std::path::Path
//! use path_slash::PathExt as _;
//! // Trait for extending std::path::PathBuf
//! use path_slash::PathBufExt as _;
//! // Trait for extending std::borrow::Cow
//! use path_slash::CowExt as _;
//!
//! #[cfg(target_os = "windows")]
//! {
//!     // Convert from `Path`
//!     assert_eq!(
//!         Path::new(r"foo\bar\piyo.txt").to_slash().unwrap(),
//!         "foo/bar/piyo.txt",
//!     );
//!
//!     // Convert to/from PathBuf
//!     let p = PathBuf::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
//!     assert_eq!(p.to_slash().unwrap(), "foo/bar/piyo.txt");
//!
//!     // Convert to Cow<'_, Path>
//!     let p = Cow::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, Cow::<Path>::Owned(PathBuf::from(r"foo\bar\piyo.txt")));
//! }
//!
//! #[cfg(not(target_os = "windows"))]
//! {
//!     // Convert from `Path`
//!     assert_eq!(
//!         Path::new("foo/bar/piyo.txt").to_slash().unwrap(),
//!         "foo/bar/piyo.txt",
//!     );
//!
//!     // Convert to/from PathBuf
//!     let p = PathBuf::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, PathBuf::from("foo/bar/piyo.txt"));
//!     assert_eq!(p.to_slash().unwrap(), "foo/bar/piyo.txt");
//!
//!     // Convert to Cow<'_, Path>
//!     let p = Cow::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, Cow::Borrowed(Path::new("foo/bar/piyo.txt")));
//! }
//! ```
#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use std::os::windows::ffi::OsStrExt;

    // Workaround for Windows. There is no way to extract raw byte sequence from `OsStr` (in `Path`).
    // And `OsStr::to_string_lossy` may cause extra heap allocation.
    pub(crate) fn ends_with_main_sep(p: &Path) -> bool {
        p.as_os_str().encode_wide().last() == Some(MAIN_SEPARATOR as u16)
    }
}

fn str_to_path(s: &str, sep: char) -> Cow<'_, Path> {
    let mut buf = String::new();

    for (i, c) in s.char_indices() {
        if c == sep {
            if buf.is_empty() {
                buf.reserve(s.len());
                buf.push_str(&s[..i]);
            }
            buf.push(MAIN_SEPARATOR);
        } else if !buf.is_empty() {
            buf.push(c);
        }
    }

    if buf.is_empty() {
        Cow::Borrowed(Path::new(s))
    } else {
        Cow::Owned(PathBuf::from(buf))
    }
}

fn str_to_pathbuf<S: AsRef<str>>(s: S, sep: char) -> PathBuf {
    let s = s
        .as_ref()
        .chars()
        .map(|c| if c == sep { MAIN_SEPARATOR } else { c })
        .collect::<String>();
    PathBuf::from(s)
}

/// Trait to extend [`Path`].
///
/// ```
/// # use std::path::Path;
/// # use std::borrow::Cow;
/// use path_slash::PathExt as _;
///
/// assert_eq!(
///     Path::new("foo").to_slash(),
///     Some(Cow::Borrowed("foo")),
/// );
/// ```
pub trait PathExt {
    fn to_slash(&self) -> Option<Cow<'_, str>>;
    fn to_slash_lossy(&self) -> Cow<'_, str>;
}

impl PathExt for Path {
    /// Convert the file path into slash path as UTF-8 string. This method is similar to
    /// [`Path::to_string_lossy`], but the path separator is fixed to '/'.
    ///
    /// Any file path separators in the file path are replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// ```
    /// # use std::path::Path;
    /// use path_slash::PathExt as _;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash_lossy(), "foo/bar/piyo.txt");
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn to_slash_lossy(&self) -> Cow<'_, str> {
        self.to_string_lossy()
    }
    #[cfg(target_os = "windows")]
    fn to_slash_lossy(&self) -> Cow<'_, str> {
        use std::path::Component;

        let mut buf = String::new();
        for c in self.components() {
            match c {
                Component::RootDir => { /* empty */ }
                Component::CurDir => buf.push('.'),
                Component::ParentDir => buf.push_str(".."),
                Component::Prefix(prefix) => {
                    buf.push_str(&prefix.as_os_str().to_string_lossy());
                    // C:\foo is [Prefix, RootDir, Normal]. Avoid C://
                    continue;
                }
                Component::Normal(s) => buf.push_str(&s.to_string_lossy()),
            }
            buf.push('/');
        }

        if !windows::ends_with_main_sep(self) && buf != "/" && buf.ends_with('/') {
            buf.pop(); // Pop last '/'
        }

        Cow::Owned(buf)
    }

    /// Convert the file path into slash path as UTF-8 string. This method is similar to
    /// [`Path::to_str`], but the path separator is fixed to '/'.
    ///
    /// Any file path separators in the file path are replaced with '/'. Only when the replacement
    /// happens, heap allocation happens and `Cow::Owned` is returned.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// ```
    /// # use std::path::Path;
    /// # use std::borrow::Cow;
    /// use path_slash::PathExt as _;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash(), Some(Cow::Borrowed("foo/bar/piyo.txt")));
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn to_slash(&self) -> Option<Cow<'_, str>> {
        self.to_str().map(Cow::Borrowed)
    }
    #[cfg(target_os = "windows")]
    fn to_slash(&self) -> Option<Cow<'_, str>> {
        use std::path::Component;

        let mut buf = String::new();
        for c in self.components() {
            match c {
                Component::RootDir => { /* empty */ }
                Component::CurDir => buf.push('.'),
                Component::ParentDir => buf.push_str(".."),
                Component::Prefix(prefix) => {
                    buf.push_str(prefix.as_os_str().to_str()?);
                    // C:\foo is [Prefix, RootDir, Normal]. Avoid C://
                    continue;
                }
                Component::Normal(s) => buf.push_str(s.to_str()?),
            }
            buf.push('/');
        }

        if !windows::ends_with_main_sep(self) && buf != "/" && buf.ends_with('/') {
            buf.pop(); // Pop last '/'
        }

        Some(Cow::Owned(buf))
    }
}

/// Trait to extend [`PathBuf`].
///
/// ```
/// # use std::path::PathBuf;
/// use path_slash::PathBufExt;
///
/// assert_eq!(
///     PathBuf::from_slash("foo/bar/piyo.txt").to_slash().unwrap(),
///     "foo/bar/piyo.txt",
/// );
/// ```
pub trait PathBufExt {
    fn from_slash<S: AsRef<str>>(s: S) -> Self;
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self;
    fn from_backslash<S: AsRef<str>>(s: S) -> Self;
    fn from_backslash_lossy<S: AsRef<OsStr>>(s: S) -> Self;
    fn to_slash(&self) -> Option<Cow<'_, str>>;
    fn to_slash_lossy(&self) -> Cow<'_, str>;
}

impl PathBufExt for PathBuf {
    /// Convert the slash path (path separated with '/') to [`PathBuf`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// The replacements only happen on Windows since the file path separators on Unix-like OS are
    /// the same as '/'.
    ///
    /// On non-Windows OS, it is simply equivalent to [`PathBuf::from`].
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// use path_slash::PathBufExt;
    ///
    /// let p = PathBuf::from_slash("foo/bar/piyo.txt");
    ///
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// assert_eq!(p, PathBuf::from("foo/bar/piyo.txt"));
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn from_slash<S: AsRef<str>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }
    #[cfg(target_os = "windows")]
    fn from_slash<S: AsRef<str>>(s: S) -> Self {
        str_to_pathbuf(s, '/')
    }

    /// Convert the [`OsStr`] slash path (path separated with '/') to [`PathBuf`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// The replacements only happen on Windows since the file path separators on Unix-like OS are
    /// the same as '/'.
    ///
    /// On Windows, any non-Unicode sequences are replaced with U+FFFD while the conversion.
    /// On non-Windows OS, it is simply equivalent to [`PathBuf::from`] and there is no
    /// loss while conversion.
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use std::ffi::OsStr;
    /// use path_slash::PathBufExt;
    ///
    /// let s: &OsStr = "foo/bar/piyo.txt".as_ref();
    /// let p = PathBuf::from_slash_lossy(s);
    ///
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// assert_eq!(p, PathBuf::from("foo/bar/piyo.txt"));
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }
    #[cfg(target_os = "windows")]
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        Self::from_slash(&s.as_ref().to_string_lossy())
    }

    /// Convert the backslash path (path separated with '\\') to [`PathBuf`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator.
    /// The replacements only happen on non-Windows.
    #[cfg(not(target_os = "windows"))]
    fn from_backslash<S: AsRef<str>>(s: S) -> Self {
        str_to_pathbuf(s, '\\')
    }
    #[cfg(target_os = "windows")]
    fn from_backslash<S: AsRef<str>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }

    /// Convert the [`OsStr`] backslash path (path separated with '\\') to [`PathBuf`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator.
    #[cfg(not(target_os = "windows"))]
    fn from_backslash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        s.as_ref().to_string_lossy().replace('\\', "/").into()
    }
    #[cfg(target_os = "windows")]
    fn from_backslash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }

    /// Convert the file path into slash path as UTF-8 string. This method is similar to
    /// [`Path::to_string_lossy`], but the path separator is fixed to '/'.
    ///
    /// Any file path separators in the file path are replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// use path_slash::PathBufExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = PathBuf::from(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = PathBuf::from("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash_lossy(), "foo/bar/piyo.txt");
    /// ```
    fn to_slash_lossy(&self) -> Cow<'_, str> {
        self.as_path().to_slash_lossy()
    }

    /// Convert the file path into slash path as UTF-8 string. This method is similar to
    /// [`Path::to_str`], but the path separator is fixed to '/'.
    ///
    /// Any file path separators in the file path are replaced with '/'. Only when the replacement
    /// happens, heap allocation happens and `Cow::Owned` is returned.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// # use std::borrow::Cow;
    /// use path_slash::PathBufExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = PathBuf::from(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = PathBuf::from("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash(), Some(Cow::Borrowed("foo/bar/piyo.txt")));
    /// ```
    fn to_slash(&self) -> Option<Cow<'_, str>> {
        self.as_path().to_slash()
    }
}

/// Trait to extend [`std::borrow::Cow`].
///
/// ```
/// # use std::path::Path;
/// # use std::borrow::Cow;
/// use path_slash::{CowExt as _, PathExt as _};
///
/// assert_eq!(
///     Cow::from_slash("foo/bar/piyo.txt").to_slash_lossy(),
///     "foo/bar/piyo.txt",
/// );
/// ```
pub trait CowExt<'a> {
    fn from_slash(s: &'a str) -> Self;
    fn from_slash_lossy(s: &'a OsStr) -> Self;
    fn from_backslash(s: &'a str) -> Self;
    fn from_backslash_lossy(s: &'a OsStr) -> Self;
}

impl<'a> CowExt<'a> for Cow<'a, Path> {
    /// Convert the slash path (path separated with '/') to [`Cow`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// Heap allocation may only happen on Windows since the file path separators on Unix-like OS
    /// are the same as '/'.
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use std::path::Path;
    /// use path_slash::CowExt;
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// assert_eq!(
    ///     Cow::from_slash("foo/bar/piyo.txt"),
    ///     Path::new("foo/bar/piyo.txt"),
    /// );
    ///
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(
    ///     Cow::from_slash("foo/bar/piyo.txt"),
    ///     Path::new(r"foo\\bar\\piyo.txt"),
    /// );
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn from_slash(s: &'a str) -> Self {
        Cow::Borrowed(Path::new(s))
    }
    #[cfg(target_os = "windows")]
    fn from_slash(s: &'a str) -> Self {
        str_to_path(s, '/')
    }

    /// Convert the [`OsStr`] slash path (path separated with '/') to [`Cow`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// Heap allocation may only happen on Windows since the file path separators on Unix-like OS
    /// are the same as '/'.
    ///
    /// On Windows, any non-Unicode sequences are replaced with U+FFFD while the conversion.
    /// On non-Windows OS, there is no loss while conversion.
    #[cfg(not(target_os = "windows"))]
    fn from_slash_lossy(s: &'a OsStr) -> Self {
        Cow::Borrowed(Path::new(s))
    }
    #[cfg(target_os = "windows")]
    fn from_slash_lossy(s: &'a OsStr) -> Self {
        match s.to_string_lossy() {
            Cow::Borrowed(s) => str_to_path(s, '/'),
            Cow::Owned(s) => Cow::Owned(str_to_pathbuf(&s, '/')),
        }
    }

    /// Convert the backslash path (path separated with '\\') to [`Cow`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator. Heap allocation may
    /// only happen on non-Windows.
    ///
    /// ```
    /// # use std::borrow::Cow;
    /// # use std::path::Path;
    /// use path_slash::CowExt;
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// assert_eq!(
    ///     Cow::from_backslash(r"foo\\bar\\piyo.txt"),
    ///     Path::new("foo/bar/piyo.txt"),
    /// );
    ///
    /// #[cfg(target_os = "windows")]
    /// assert_eq!(
    ///     Cow::from_backslash(r"foo\\bar\\piyo.txt"),
    ///     Path::new(r"foo\\bar\\piyo.txt"),
    /// );
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn from_backslash(s: &'a str) -> Self {
        str_to_path(s, '\\')
    }
    #[cfg(target_os = "windows")]
    fn from_backslash(s: &'a str) -> Self {
        Cow::Borrowed(Path::new(s))
    }

    /// Convert the [`OsStr`] backslash path (path separated with '\\') to [`Cow`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator. Heap allocation may
    /// only happen on non-Windows.
    #[cfg(not(target_os = "windows"))]
    fn from_backslash_lossy(s: &'a OsStr) -> Self {
        match s.to_string_lossy() {
            Cow::Borrowed(s) => str_to_path(s, '\\'),
            Cow::Owned(s) => Cow::Owned(str_to_pathbuf(&s, '\\')),
        }
    }
    #[cfg(target_os = "windows")]
    fn from_backslash_lossy(s: &'a OsStr) -> Self {
        Cow::Borrowed(Path::new(s))
    }
}

#[cfg(test)]
mod test;
