Rust library to convert a file path from/to slash path
======================================================
[![CI on Linux and macOS][travis-ci-badge]][travis-ci]
[![CI on Windows][appveyor-badge]][appveyor]

[`path-slash`][crates-io] is a tiny library to convert a file path (e.g. `foo/bar`, `foo\bar` or
`C:\\foo\bar`) from/to slash path (e.g. `foo/bar`, `C://foo/bar`).

In Unix-like OS, path separator is slash `/` by default. So any conversion is not necessary. But on
Windows, file path separator `\` needs to be replaced with slash `/` (and of course `\` for escaping
character should not be replaced).

This package was inspired by Go's [`path/filepath.FromSlash`](https://golang.org/pkg/path/filepath/#FromSlash)
and [`path/filepath.ToSlash`](https://golang.org/pkg/path/filepath/#ToSlash).

## Usage

`path_slash::PathExt` and `path_slash::PathBufExt` traits are defined. By using them, `std::path::Path`
and `std::path::PathBuf` gains some methods and associated functions

- `PathExt`
  - `Path::to_slash(&self) -> Option<String>`
  - `Path::to_slash_lossy(&self) -> String`
- `PathBufExt`
  - `PathBuf::from_slash<S: AsRef<str>>(s: S) -> PathBuf`
  - `PathBuf::from_slash_lossy<S: AsRef<OsStr>>(s: S) -> PathBuf`
  - `PathBuf::to_slash(&self) -> Option<String>`
  - `PathBuf::to_slash_lossy(&self) -> String`

```rust
fn example_path_ext() {
    // Trait for extending std::path::Path
    use path_slash::PathExt;

    // On Windows
    assert_eq!(Path::new(r"foo\bar\piyo.txt").to_slash(), Some("foo/bar/piyo.txt".to_string()));
    assert_eq!(Path::new(r"C:\\foo\bar\piyo.txt").to_slash(), Some("C://foo/bar/piyo.txt".to_string()));
}

fn example_pathbuf_ext() {
    // Trait for extending std::path::PathBuf
    use path_slash::PathBufExt;

    // On Windows
    let p = PathBuf::from_slash("foo/bar/piyo.txt");
    assert_eq!(p, PathBuf::from(r"foo\bar\piyo.txt"));
    assert_eq!(p.to_slash(), Some("foo/bar/piyo.txt".to_string()));
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

[doc]: https://docs.rs/path-slash
[crates-io]: https://crates.io/crates/path-slash
[appveyor-badge]: https://ci.appveyor.com/api/projects/status/44t8q0viea89fm2e/branch/master?svg=true
[appveyor]: https://ci.appveyor.com/project/rhysd/path-slash/branch/master
[travis-ci-badge]: https://travis-ci.org/rhysd/path-slash.svg?branch=master
[travis-ci]: https://travis-ci.org/rhysd/path-slash
