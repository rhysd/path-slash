use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub trait PathExt {
    fn to_slash(&self) -> Option<String>;
    fn to_slash_lossy(&self) -> String;
}

impl PathExt for Path {
    #[cfg(not(target_os = "windows"))]
    fn to_slash_lossy(&self) -> String {
        self.to_string_lossy().to_string()
    }

    #[cfg(target_os = "windows")]
    fn to_slash_lossy(&self) -> String {
        use std::path;

        let mut first = true;
        let mut buf = String::new();
        for c in self.components() {
            if first {
                first = false;
            } else {
                buf.push('/');
            }
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
                }
                path::Component::Normal(ref s) => match s.to_str() {
                    Some(ref s) => buf.push_str(s),
                    None => buf.push_str(&s.to_string_lossy()),
                },
            }
            buf.push('/');
        }
        buf
    }

    #[cfg(not(target_os = "windows"))]
    fn to_slash(&self) -> Option<String> {
        self.to_str().map(str::to_string)
    }

    #[cfg(target_os = "windows")]
    fn to_slash(&self) -> Option<String> {
        use std::path;
        let components = self
            .components()
            .map(|c| match c {
                path::Component::RootDir => Some(""),
                path::Component::CurDir => Some("."),
                path::Component::ParentDir => Some(".."),
                path::Component::Prefix(ref p) => p.as_os_str().to_str(),
                path::Component::Normal(ref s) => s.to_str(),
            })
            .collect::<Option<Vec<_>>>();
        components.map(|v| v.join("/"))
    }
}

pub trait PathBufExt {
    fn from_slash<S: AsRef<str>>(s: S) -> Self;
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self;
    fn to_slash(&self) -> Option<String>;
    fn to_slash_lossy(&self) -> String;
}

impl PathBufExt for PathBuf {
    #[cfg(not(target_os = "windows"))]
    fn from_slash<S: AsRef<str>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }

    #[cfg(not(target_os = "windows"))]
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        PathBuf::from(s.as_ref())
    }

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

    #[cfg(target_os = "windows")]
    fn from_slash_lossy<S: AsRef<OsStr>>(s: S) -> Self {
        Self::from_slash(s.as_ref().to_string_lossy().chars().as_str())
    }

    fn to_slash_lossy(&self) -> String {
        self.as_path().to_slash_lossy()
    }

    fn to_slash(&self) -> Option<String> {
        self.as_path().to_slash()
    }
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test;
