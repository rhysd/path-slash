use super::{PathBufExt as _, PathExt as _};
use lazy_static::lazy_static;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path;
use std::path::PathBuf;

lazy_static! {
    static ref FROM_SLASH_TESTS: Vec<(String, PathBuf)> = {
        [
            ("", ""),
            ("/", "/"),
            ("//", "/"),
            ("foo", "foo"),
            ("/foo", "/foo"),
            ("foo/", "foo"),
            ("/foo/", "/foo"),
            ("./foo", "./foo"),
            ("../foo", "../foo"),
            ("foo/.", "foo/."),
            ("foo/..", "foo/.."),
            ("foo/bar", "foo/bar"),
            ("foo//bar", "foo/bar"),
            ("foo/../bar", "foo/../bar"),
            ("foo/./bar", "foo/./bar"),
        ]
        .iter()
        .map(|item| {
            let (input, expected) = item;
            let expected = if cfg!(target_os = "windows") {
                let s = expected
                    .chars()
                    .map(|c| match c {
                        '/' => path::MAIN_SEPARATOR,
                        _ => c,
                    })
                    .collect::<String>();
                PathBuf::from(s)
            } else {
                PathBuf::from(expected)
            };
            (input.to_string(), expected)
        })
        .collect::<Vec<_>>()
    };
}

#[test]
fn from_slash() {
    for (input, expected) in FROM_SLASH_TESTS.iter() {
        assert_eq!(&PathBuf::from_slash(input), expected);
    }
}

#[test]
fn from_slash_lossy() {
    for (input, expected) in FROM_SLASH_TESTS.iter() {
        let input: &OsStr = input.as_ref();
        assert_eq!(&PathBuf::from_slash_lossy(input), expected);
    }
}

#[test]
fn from_backslash() {
    for (input, expected) in FROM_SLASH_TESTS.iter() {
        let input = input.replace('/', r"\");
        assert_eq!(&PathBuf::from_backslash(input), expected);
    }
}

#[test]
fn from_backslash_lossy() {
    for (input, expected) in FROM_SLASH_TESTS.iter() {
        let input = input.replace('/', r"\");
        let input: &OsStr = input.as_ref();
        assert_eq!(&PathBuf::from_backslash_lossy(input), expected);
    }
}

lazy_static! {
    static ref TO_SLASH_TESTS: Vec<(PathBuf, String)> = {
        [
            "",
            "/",
            "foo",
            "/foo",
            "foo",
            "/foo",
            "./foo",
            "../foo",
            "foo/..",
            "foo/bar",
            "foo/../bar",
        ]
        .iter()
        .map(|expected| {
            let input = if cfg!(target_os = "windows") {
                let s = expected
                    .chars()
                    .map(|c| match c {
                        '/' => path::MAIN_SEPARATOR,
                        _ => c,
                    })
                    .collect::<String>();
                PathBuf::from(s)
            } else {
                PathBuf::from(expected)
            };
            (input, expected.to_string())
        })
        .collect::<Vec<_>>()
    };
}

#[test]
fn to_slash_path() {
    for (input, expected) in TO_SLASH_TESTS.iter() {
        assert_eq!(
            input.as_path().to_slash(),
            Some(Cow::Borrowed(expected.as_str()))
        );
    }
}

#[test]
fn to_slash_pathbuf() {
    for (input, expected) in TO_SLASH_TESTS.iter() {
        assert_eq!(input.to_slash(), Some(Cow::Borrowed(expected.as_str())));
    }
}

#[test]
fn to_slash_lossy_path() {
    for (input, expected) in TO_SLASH_TESTS.iter() {
        assert_eq!(&input.as_path().to_slash_lossy(), expected);
    }
}

#[test]
fn to_slash_lossy_pathbuf() {
    for (input, expected) in TO_SLASH_TESTS.iter() {
        assert_eq!(&input.to_slash_lossy(), expected);
    }
}

#[test]
fn from_slash_to_slash() {
    for (_, path) in TO_SLASH_TESTS.iter() {
        assert_eq!(
            PathBuf::from_slash(path).to_slash(),
            Some(Cow::Borrowed(path.as_str()))
        );
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;

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
}
