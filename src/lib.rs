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
//!         Path::new(r"foo\bar\piyo.txt").to_slash(),
//!         Some("foo/bar/piyo.txt".to_string()),
//!     );
//!     assert_eq!(
//!         Path::new(r"C:\foo\bar\piyo.txt").to_slash(),
//!         Some("C:/foo/bar/piyo.txt".to_string()),
//!     );
//!
//!     let p = PathBuf::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
//!     assert_eq!(p.to_slash(), Some("foo/bar/piyo.txt".to_string()));
//! }
//!
//! #[cfg(not(target_os = "windows"))]
//! {
//!     assert_eq!(
//!         Path::new("foo/bar/piyo.txt").to_slash(),
//!         Some("foo/bar/piyo.txt".to_string()),
//!     );
//!     assert_eq!(
//!         Path::new("/foo/bar/piyo.txt").to_slash(),
//!         Some("/foo/bar/piyo.txt".to_string()),
//!     );
//!
//!     let p = PathBuf::from_slash("foo/bar/piyo.txt");
//!     assert_eq!(p, PathBuf::from(r"foo/bar/piyo.txt"));
//!     assert_eq!(p.to_slash(), Some("foo/bar/piyo.txt".to_string()));
//! }
//! ```
#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Trait to extend [`std::path::Path`].
///
/// ```
/// use path_slash::PathExt;
///
/// assert_eq!(
///     std::path::Path::new("foo").to_slash(),
///     Some("foo".to_string()),
/// );
/// ```
pub trait PathExt {
    fn to_slash(&self) -> Option<String>;
    fn to_slash_lossy(&self) -> String;
}

impl PathExt for Path {
    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// On non-Windows OS, it is equivalent to `to_string_lossy().to_string()`
    ///
    /// ```
    /// use std::path::Path;
    /// use path_slash::PathExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash_lossy(), "foo/bar/piyo.txt".to_string());
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn to_slash_lossy(&self) -> String {
        self.to_string_lossy().to_string()
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// On non-Windows OS, it is equivalent to `.to_string_lossy().to_string()`.
    ///
    /// ```
    /// use std::path::Path;
    /// use path_slash::PathExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash_lossy(), "foo/bar/piyo.txt".to_string());
    /// ```
    #[cfg(target_os = "windows")]
    fn to_slash_lossy(&self) -> String {
        use std::path;

        let mut buf = String::new();
        for c in self.components() {
            match c {
                path::Component::RootDir => { /* empty */ }
                path::Component::CurDir => buf.push('.'),
                path::Component::ParentDir => buf.push_str(".."),
                path::Component::Prefix(ref prefix) => {
                    let s = prefix.as_os_str();
                    match s.to_str() {
                        Some(ref s) => buf.push_str(s),
                        None => buf.push_str(&s.to_string_lossy()),
                    }
                    // C:\foo is [Prefix, RootDir, Normal]. Avoid C://
                    continue;
                }
                path::Component::Normal(ref s) => match s.to_str() {
                    Some(ref s) => buf.push_str(s),
                    None => buf.push_str(&s.to_string_lossy()),
                },
            }
            buf.push('/');
        }

        if buf != "/" {
            buf.pop(); // Pop last '/'
        }

        buf
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// On non-Windows OS, it is equivalent to `.to_str().map(str::to_string)`
    ///
    /// ```
    /// use std::path::Path;
    /// use path_slash::PathExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash(), Some("foo/bar/piyo.txt".to_string()));
    /// ```
    #[cfg(not(target_os = "windows"))]
    fn to_slash(&self) -> Option<String> {
        self.to_str().map(str::to_string)
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// On non-Windows OS, it is equivalent to `.to_str().map(str::to_string)`
    ///
    /// ```
    /// use std::path::Path;
    /// use path_slash::PathExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = Path::new(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = Path::new("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash(), Some("foo/bar/piyo.txt".to_string()));
    /// ```
    #[cfg(target_os = "windows")]
    fn to_slash(&self) -> Option<String> {
        use std::path;

        let mut buf = String::new();
        for c in self.components() {
            match c {
                path::Component::RootDir => { /* empty */ }
                path::Component::CurDir => buf.push('.'),
                path::Component::ParentDir => buf.push_str(".."),
                path::Component::Prefix(ref prefix) => {
                    if let Some(s) = prefix.as_os_str().to_str() {
                        buf.push_str(s);
                        // C:\foo is [Prefix, RootDir, Normal]. Avoid C://
                        continue;
                    } else {
                        return None;
                    }
                }
                path::Component::Normal(ref s) => {
                    if let Some(s) = s.to_str() {
                        buf.push_str(s);
                    } else {
                        return None;
                    }
                }
            }
            buf.push('/');
        }

        if buf != "/" {
            buf.pop(); // Pop last '/'
        }

        Some(buf)
    }
}

/// Trait to extend [`std::path::PathBuf`].
///
/// ```
/// use path_slash::PathBufExt;
///
/// assert_eq!(
///     std::path::PathBuf::from_slash("foo/bar/piyo.txt").to_slash(),
///     Some("foo/bar/piyo.txt".to_string()),
/// );
/// ```
pub trait PathBufExt {
    fn from_slash<S: AsRef<str>>(s: S) -> Self;
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self;
    fn to_slash(&self) -> Option<String>;
    fn to_slash_lossy(&self) -> String;
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
    /// use std::path::PathBuf;
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
    /// use std::path::PathBuf;
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
        use std::path;

        let s = s
            .as_ref()
            .chars()
            .map(|c| match c {
                '/' => path::MAIN_SEPARATOR,
                c => c,
            })
            .collect::<String>();
        PathBuf::from(s)
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
    /// use std::ffi::OsStr;
    /// use std::path::PathBuf;
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
    /// use std::ffi::OsStr;
    /// use std::path::PathBuf;
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
        Self::from_slash(s.as_ref().to_string_lossy().chars().as_str())
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// Any non-Unicode sequences are replaced with U+FFFD.
    ///
    /// On non-Windows OS, it is equivalent to `to_string_lossy().to_string()`
    ///
    /// ```
    /// use path_slash::PathBufExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = std::path::PathBuf::from(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = std::path::PathBuf::from("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash_lossy(), "foo/bar/piyo.txt".to_string());
    /// ```
    fn to_slash_lossy(&self) -> String {
        self.as_path().to_slash_lossy()
    }

    /// Convert the file path into slash path as UTF-8 string.
    ///
    /// Any file path separators in the file path is replaced with '/'.
    /// When the path contains non-Unicode sequence, this method returns None.
    ///
    /// On non-Windows OS, it is equivalent to `.to_str().map(std::to_string())`
    ///
    /// ```
    /// use path_slash::PathBufExt;
    ///
    /// #[cfg(target_os = "windows")]
    /// let s = std::path::PathBuf::from(r"foo\bar\piyo.txt");
    ///
    /// #[cfg(not(target_os = "windows"))]
    /// let s = std::path::PathBuf::from("foo/bar/piyo.txt");
    ///
    /// assert_eq!(s.to_slash(), Some("foo/bar/piyo.txt".to_string()));
    /// ```
    fn to_slash(&self) -> Option<String> {
        self.as_path().to_slash()
    }
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test;
