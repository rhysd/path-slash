Rust library to convert a file path from/to slash path
======================================================
[![crates.io][crates-io-badge]][crates-io]
[![documentation][doc-badge]][doc]
[![CI][ci-badge]][ci]

[`path-slash`][crates-io] is a tiny library to convert a file path (e.g. `foo/bar`, `foo\bar` or
`C:\foo\bar`) from/to slash path (e.g. `foo/bar`, `C:/foo/bar`).

On Unix-like OS, path separator is slash `/` by default. One may want to convert a Windows path. But on
Windows, file path separator `\` needs to be replaced with slash `/` (and of course `\`s for escaping
characters should not be replaced).

This package was inspired by Go's [`path/filepath.FromSlash`](https://golang.org/pkg/path/filepath/#FromSlash)
and [`path/filepath.ToSlash`](https://golang.org/pkg/path/filepath/#ToSlash).

## Usage

`path_slash::PathExt` and `path_slash::PathBufExt` traits are defined. By using them, `std::path::Path`,
`std::path::PathBuf` and `Cow<'_, Path>` gain some methods and associated functions.

- `PathExt`
  - `Path::to_slash(&self) -> Option<Cow<'_, Path>>`
  - `Path::to_slash_lossy(&self) -> Cow<'_, Path>`
- `PathBufExt`
  - `PathBuf::from_slash<S: AsRef<str>>(s: S) -> PathBuf`
  - `PathBuf::from_slash_lossy<S: AsRef<OsStr>>(s: S) -> PathBuf`
  - `PathBuf::from_backslash<S: AsRef<str>>(s: S) -> PathBuf`
  - `PathBuf::from_backslash_lossy<S: AsRef<OsStr>>(s: S) -> PathBuf`
  - `PathBuf::to_slash(&self) -> Option<Cow<'_, Path>>`
  - `PathBuf::to_slash_lossy(&self) -> Cow<'_, Path>`
- `CowExt`
  - `fn from_slash(s: &str) -> Self`
  - `fn from_slash_lossy(s: &OsStr) -> Self`
  - `fn from_backslash(s: &str) -> Self`
  - `fn from_backslash_lossy(s: &OsStr) -> Self`

```rust
fn example_path_ext() {
    // Trait for extending std::path::Path
    use path_slash::PathExt as _;

    // On Windows
    assert_eq!(
        Path::new(r"foo\bar\piyo.txt").to_slash(),
        Some("foo/bar/piyo.txt".to_string()),
    );
}

fn example_pathbuf_ext() {
    // Trait for extending std::path::PathBuf
    use path_slash::PathBufExt as _;

    // On Windows
    let p = PathBuf::from_slash("foo/bar/piyo.txt");
    assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
    assert_eq!(p.to_slash().unwrap(), "foo/bar/piyo.txt".to_string());
}

fn example_cow_ext() {
    // Trait for extending std::borrow::Cow<'_, Path>
    use path_slash::CowExt as _;

    let p = Cow::from_slash("foo/bar/piyo.txt");
    // On Windows
    assert_eq!(p, Cow::Owned(PathBuf::from(r"foo\bar\piyo.txt")));
    // On non-Windows
    assert_eq!(p, Cow::Borrowed(Path::new("foo/bar/piyo.txt")));
}
```

Please read [documents][doc] for more details.

## Installation

Add `path-slash` to dependencies:

```toml
[dependencies]
path-slash = "0.x"
```

## License

[the MIT License](LICENSE.txt)

[doc-badge]: https://docs.rs/path-slash/badge.svg
[doc]: https://docs.rs/path-slash
[crates-io-badge]: https://img.shields.io/crates/v/path-slash.svg
[crates-io]: https://crates.io/crates/path-slash
[ci]: https://github.com/rhysd/path-slash/actions?query=workflow%3ACI
[ci-badge]: https://github.com/rhysd/path-slash/workflows/CI/badge.svg?branch=master&event=push
