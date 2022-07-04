#![cfg(target_os = "windows")]

use path_slash::{CowExt as _, PathBufExt as _, PathExt as _};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[test]
fn with_driver_letter_to_slash() {
    let path = PathBuf::from_slash("C:/foo/bar");
    assert_eq!(path, PathBuf::from(r"C:\foo\bar"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, "C:/foo/bar");
}

#[test]
fn with_drive_letter_to_slash_lossy() {
    let path = PathBuf::from_slash("C:/foo/bar");
    assert_eq!(path, PathBuf::from(r"C:\foo\bar"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, "C:/foo/bar");
}

#[test]
fn with_drive_letter_but_no_path_to_slash() {
    let path = PathBuf::from_slash("C:");
    assert_eq!(path, PathBuf::from(r"C:"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, "C:");
}

#[test]
fn with_drive_letter_but_no_path_to_slash_lossy() {
    let path = PathBuf::from_slash("C:");
    assert_eq!(path, PathBuf::from(r"C:"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, "C:");
}

#[test]
fn with_verbatim_drive_letter_to_slash() {
    let path = PathBuf::from_slash(r"\\?\C:/foo/bar");
    assert_eq!(path, PathBuf::from(r"\\?\C:\foo\bar"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, r"\\?\C:/foo/bar");
}

#[test]
fn with_verbatim_drive_letter_to_slash_lossy() {
    let path = PathBuf::from_slash(r"\\?\C:/foo/bar");
    assert_eq!(path, PathBuf::from(r"\\?\C:\foo\bar"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, r"\\?\C:/foo/bar");
}

#[test]
fn with_unc_prefix_to_slash() {
    let path = PathBuf::from_slash(r"\\server\share/foo/bar");
    assert_eq!(path, PathBuf::from(r"\\server\share\foo\bar"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, r"\\server\share/foo/bar");
}

#[test]
fn with_unc_prefix_to_slash_lossy() {
    let path = PathBuf::from_slash(r"\\server\share/foo/bar");
    assert_eq!(path, PathBuf::from(r"\\server\share\foo\bar"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, r"\\server\share/foo/bar");
}

#[test]
fn with_unc_prefix_but_no_path_to_slash() {
    let path = PathBuf::from_slash(r"\\server\share");
    assert_eq!(path, PathBuf::from(r"\\server\share"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, r"\\server\share");
}

#[test]
fn with_unc_prefix_but_no_path_to_slash_lossy() {
    let path = PathBuf::from_slash(r"\\server\share");
    assert_eq!(path, PathBuf::from(r"\\server\share"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, r"\\server\share");
}

#[test]
fn with_verbatim_unc_prefix_to_slash() {
    let path = PathBuf::from_slash(r"\\?\UNC\server\share/foo/bar");
    assert_eq!(path, PathBuf::from(r"\\?\UNC\server\share\foo\bar"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, r"\\?\UNC\server\share/foo/bar");
}

#[test]
fn with_verbatim_unc_prefix_to_slash_lossy() {
    let path = PathBuf::from_slash(r"\\?\UNC\server\share/foo/bar");
    assert_eq!(path, PathBuf::from(r"\\?\UNC\server\share\foo\bar"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, r"\\?\UNC\server\share/foo/bar");
}

#[test]
fn with_verbatim_unc_prefix_but_no_path_to_slash() {
    let path = PathBuf::from_slash(r"\\?\UNC\server\share");
    assert_eq!(path, PathBuf::from(r"\\?\UNC\server\share"));
    let slash = path.to_slash().unwrap();
    assert_eq!(slash, r"\\?\UNC\server\share");
}

#[test]
fn with_verbatim_unc_prefix_but_no_path_to_slash_lossy() {
    let path = PathBuf::from_slash(r"\\?\UNC\server\share");
    assert_eq!(path, PathBuf::from(r"\\?\UNC\server\share"));
    let slash = path.to_slash_lossy();
    assert_eq!(slash, r"\\?\UNC\server\share");
}

const UTF16_TEST_CASES: &[(&str, &str)] = &[
    (
        // あ\い\う\え\お
        "\x30\x42\x00\x5c\x30\x44\x00\x5c\x30\x46\x00\x5c\x30\x48\x00\x5c\x30\x4a",
        // あ/い/う/え/お
        "\x30\x42\x00\x2f\x30\x44\x00\x2f\x30\x46\x00\x2f\x30\x48\x00\x2f\x30\x4a",
    ),
    (
        // あ\い\う\え\お\
        "\x30\x42\x00\x5c\x30\x44\x00\x5c\x30\x46\x00\x5c\x30\x48\x00\x5c\x30\x4a\x00\x5c",
        // あ/い/う/え/お/
        "\x30\x42\x00\x2f\x30\x44\x00\x2f\x30\x46\x00\x2f\x30\x48\x00\x2f\x30\x4a\x00\x2f",
    ),
];

#[test]
fn utf16_encoded_os_str_to_slash() {
    for (b, s) in UTF16_TEST_CASES {
        let p = Path::new(b);
        assert_eq!(p.to_slash().unwrap(), *s);
    }
}

#[test]
fn utf16_encoded_os_str_pathbuf_from_slash_lossy() {
    for (b, s) in UTF16_TEST_CASES {
        let p = PathBuf::from_slash_lossy(s);
        assert_eq!(p, Path::new(b));
    }
}

#[test]
fn utf16_encoded_os_str_pathbuf_from_slash() {
    for (b, s) in UTF16_TEST_CASES {
        let p = PathBuf::from_slash(s);
        assert_eq!(p, Path::new(b));
    }
}

#[test]
fn utf16_encoded_os_str_cow_from_slash_lossy() {
    for (b, s) in UTF16_TEST_CASES {
        let p = Cow::from_slash_lossy(OsStr::new(s));
        assert_eq!(p, Path::new(b));
    }
}

#[test]
fn utf16_encoded_os_str_cow_from_slash() {
    for (b, s) in UTF16_TEST_CASES {
        let p = Cow::from_slash(s);
        assert_eq!(p, Path::new(b));
    }
}
