//! A library for converting file paths to and from "slash paths."
//!
//! A "slash path" is a path whose components are always separated by `/` and never `\`.
//!
//! On Unix-like OSes, the path separator is `/`. So any conversion is not necessary.
//! But on Windows, the file path separator is `\`, and needs to be replaced with `/`. Of course, `\`s used
//! for escaping characters should not be replaced.
//!
//! For example, a file path `foo\bar\piyo.txt` can be converted to/from a slash path `foo/bar/piyo.txt`.
//!
//! This package was inspired by Go's [`path/filepath.FromSlash`](https://golang.org/pkg/path/filepath/#FromSlash)
//! and [`path/filepath.ToSlash`](https://golang.org/pkg/path/filepath/#ToSlash).
//!
//! ```rust
//! use std::path::{Path, PathBuf};
//!
//! // Trait for extending std::path::Path
//! use path_slash::PathExt;
//! // Trait for extending std::path::PathBuf
//! use path_slash::PathBufExt;
//!
//! #[cfg(target_os = "windows")]
//! {
//!     assert_eq!(
//!         Path::new(r"foo\bar\piyo.txt").to_slash().unwrap(),
//!         "foo/bar/piyo.txt",
//!     );
//!     assert_eq!(
//!         Path::new(r"C:\foo\bar\piyo.txt").to_slash().unwrap(),
//!         "C:/foo/bar/piyo.txt",
//!     );
//!
//!     let p = PathBuf::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
//!     assert_eq!(p.to_slash().unwrap(), "foo/bar/piyo.txt");
//! }
//!
//! #[cfg(not(target_os = "windows"))]
//! {
//!     assert_eq!(
//!         Path::new("foo/bar/piyo.txt").to_slash().unwrap(),
//!         "foo/bar/piyo.txt",
//!     );
//!     assert_eq!(
//!         Path::new("/foo/bar/piyo.txt").to_slash().unwrap(),
//!         "/foo/bar/piyo.txt",
//!     );
//!
//!     let p = PathBuf::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, PathBuf::from("foo/bar/piyo.txt"));
//!     assert_eq!(p.to_slash().unwrap(), "foo/bar/piyo.txt");
//! }
//! ```
#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

/// Trait to extend [`std::path::Path`].
///
/// ```
/// # use std::path::Path;
/// # use std::borrow::Cow;
/// use path_slash::PathExt;
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
    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// ```
    /// # use std::path::Path;
    /// use path_slash::PathExt;
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

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// ```
    /// # use std::path::Path;
    /// use path_slash::PathExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash_lossy(), "foo/bar/piyo.txt");
    /// ```
    #[cfg(target_os = "windows")]
    fn to_slash_lossy(&self) -> Cow<'_, str> {
        use std::path;

        let mut buf = String::new();
        let mut has_trailing_slash = false;
        for c in self.components() {
            match c {
                path::Component::RootDir => { /* empty */ }
                path::Component::CurDir => buf.push('.'),
                path::Component::ParentDir => buf.push_str(".."),
                path::Component::Prefix(ref prefix) => {
                    buf.push_str(previs.as_os_str().to_string_lossy());
                    // C:\foo is [Prefix, RootDir, Normal]. Avoid C://
                    continue;
                }
                path::Component::Normal(ref s) => buf.push_str(s.to_string_lossy()),
            }
            buf.push('/');
            has_trailing_slash = true;
        }

        if buf != "/" && has_trailing_slash {
            buf.pop(); // Pop last '/'
        }

        Cow::Owned(buf)
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// ```
    /// # use std::path::Path;
    /// # use std::borrow::Cow;
    /// use path_slash::PathExt;
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

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// On non-Windows OS, it is equivalent to `.to_str().map(str::to_string)`
    ///
    /// ```
    /// # use std::path::Path;
    /// # use std::borrow::Cow;
    /// use path_slash::PathExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash(), Some(Cow::Borrowed("foo/bar/piyo.txt")));
    /// ```
    #[cfg(target_os = "windows")]
    fn to_slash(&self) -> Option<Cow<'_, str>> {
        use std::path;

        let mut buf = String::new();
        let mut has_trailing_slash = false;
        for c in self.components() {
            match c {
                path::Component::RootDir => { /* empty */ }
                path::Component::CurDir => buf.push('.'),
                path::Component::ParentDir => buf.push_str(".."),
                path::Component::Prefix(ref prefix) => {
                    buf.push_str(prefix.as_os_str().to_str()?);
                    // C:\foo is [Prefix, RootDir, Normal]. Avoid C://
                    continue;
                }
                path::Component::Normal(ref s) => buf.push_str(s.to_str()?),
            }
            buf.push('/');
            has_trailing_slash = true;
        }

        if buf != "/" && has_trailing_slash {
            buf.pop(); // Pop last '/'
        }

        Some(Cow::Owned(buf))
    }
}

/// Trait to extend [`std::path::PathBuf`].
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
    /// Convert the slash path (path separated with '/') to [`std::path::PathBuf`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// The replacements only happen on Windows since the file path separators on other OSes are the
    /// same as '/'.
    ///
    /// On non-Windows OS, it is simply equivalent to [`std::path::PathBuf::from`].
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

    /// Convert the slash path (path separated with '/') to [`std::path::PathBuf`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// The replacements only happen on Windows since the file path separators on other OSes are the
    /// same as '/'.
    ///
    /// On non-Windows OS, it is simply equivalent to [`std::path::PathBuf::from`].
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
    #[cfg(target_os = "windows")]
    fn from_slash<S: AsRef<str>>(s: S) -> Self {
        let s = s
            .as_ref()
            .chars()
            .map(|c| match c {
                '/' => MAIN_SEPARATOR,
                c => c,
            })
            .collect::<String>();

        PathBuf::from(s)
    }

    /// Convert the backslash path (path separated with '\\') to [`std::path::PathBuf`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator.
    /// The replacements only happen on non-Windows.
    #[cfg(not(target_os = "windows"))]
    fn from_backslash<S: AsRef<str>>(s: S) -> Self {
        let s = s
            .as_ref()
            .chars()
            .map(|c| match c {
                '\\' => MAIN_SEPARATOR,
                c => c,
            })
            .collect::<String>();

        PathBuf::from(s)
    }

    /// Convert the backslash path (path separated with '\\') to [`std::path::PathBuf`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator.
    /// The replacements only happen on non-Windows.
    #[cfg(target_os = "windows")]
    fn from_backslash<S: AsRef<str>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }

    /// Convert the backslash path (path separated with '\\') to [`std::path::PathBuf`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator.
    #[cfg(not(target_os = "windows"))]
    fn from_backslash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        s.as_ref().to_string_lossy().replace('\\', "/").into()
    }

    /// Convert the backslash path (path separated with '\\') to [`std::path::PathBuf`].
    ///
    /// Any '\\' in the slash path is replaced with the file path separator.
    #[cfg(target_os = "windows")]
    fn from_backslash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }

    /// Convert the slash path (path separated with '/') to [`std::path::PathBuf`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// The replacements only happen on Windows since the file path separators on other OSes are the
    /// same as '/'.
    ///
    /// On Windows, any non-Unicode sequences are replaced with U+FFFD while the conversion.
    /// On non-Windows OS, it is simply equivalent to [`std::path::PathBuf::from`] and there is no
    /// loss while conversion.
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// use std::ffi::OsStr;
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

    /// Convert the slash path (path separated with '/') to [`std::path::PathBuf`].
    ///
    /// Any '/' in the slash path is replaced with the file path separator.
    /// The replacements only happen on Windows since the file path separators on other OSes are the
    /// same as '/'.
    ///
    /// On Windows, any non-Unicode sequences are replaced with U+FFFD while the conversion.
    /// On non-Windows OS, it is simply equivalent to [`std::path::PathBuf::from`] and there is no
    /// loss while conversion.
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// use std::ffi::OsStr;
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
    #[cfg(target_os = "windows")]
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        Self::from_slash(&s.as_ref().to_string_lossy())
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
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

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
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

#[cfg(test)]
mod test;
