//! Enviroment data

use alloc::boxed::Box;

use core_collections::borrow::ToOwned;

use ffi::{OsString, OsStr};
use fs::{self, File};
use path::{Path, PathBuf};
use string::{String, ToString};
use sys_common::AsInner;
use vec::Vec;
use error;
use fmt;
use str;

use syscall::{chdir, getcwd};

use io::{Error, Result, Read, Write};

#[allow(non_upper_case_globals)]
static mut _args: *mut Vec<&'static str> = 0 as *mut Vec<&'static str>;

/// An iterator over the arguments of a process, yielding a `String` value for each argument.
pub struct Args {
    i: usize
}

impl Iterator for Args {
    //Yes, this is supposed to be String, do not change it!
    //Only change it if https://doc.rust-lang.org/std/env/struct.Args.html changes from String
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if let Some(arg) = unsafe { (*_args).get(self.i) } {
            self.i += 1;
            Some(arg.to_string())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = if self.i <= unsafe { (*_args).len() } {
            unsafe { (*_args).len() - self.i }
        } else {
            0
        };
        (len, Some(len))
    }
}

impl ExactSizeIterator for Args {}

/// Arguments
pub fn args() -> Args {
    Args {
        i: 0
    }
}

/// Initialize arguments
pub unsafe fn args_init(args: Vec<&'static str>) {
    _args = Box::into_raw(box args);
}

/// Destroy arguments
pub unsafe fn args_destroy() {
    if _args as usize > 0 {
        drop(Box::from_raw(_args));
    }
}

/// Method to return the current directory
pub fn current_dir() -> Result<PathBuf> {
    // Return the current path
    let mut buf = [0; 4096];
    let count = getcwd(&mut buf).map_err(|x| Error::from_sys(x))?;
    Ok(PathBuf::from(unsafe { str::from_utf8_unchecked(&buf[..count]) }))
}

/// Method to return the home directory
pub fn home_dir() -> Option<PathBuf> {
    var("HOME").ok().map(PathBuf::from)
}

pub fn temp_dir() -> Option<PathBuf> {
    Some(PathBuf::from("/tmp"))
}

/// Set the current directory
pub fn set_current_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    match File::open(path) {
        Ok(file) => {
            match file.path() {
                Ok(path) => {
                    let path_str = path.as_os_str().as_inner();
                    chdir(path_str).and(Ok(())).map_err(|x| Error::from_sys(x))
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}

/// Possible errors from the `env::var` method.
#[derive(Debug)]
pub enum VarError {
    /// The specified environment variable was not set.
    NotPresent,
    /// The key or the value of the specified environment variable did not contain valid Unicode
    /// data.
    NotUnicode(OsString),
}

impl fmt::Display for VarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VarError::NotPresent => write!(f, "environment variable not found"),
            VarError::NotUnicode(ref s) => write!(f, "environment variable was not valid unicode: {:?}", s)
        }
    }
}

impl error::Error for VarError {
    fn description(&self) -> &str {
        match *self {
            VarError::NotPresent => "environment variable not found",
            VarError::NotUnicode(_) => "environment variable was not valid unicode"
        }
    }
}

/// Returns the environment variable `key` from the current process. If `key` is not valid Unicode
/// or if the variable is not present then `Err` is returned
pub fn var<K: AsRef<OsStr>>(key: K) -> ::core::result::Result<String, VarError> {
    if let Some(key_str) = key.as_ref().to_str() {
        if ! key_str.is_empty() {
            let mut file = try!(File::open(&("env:".to_owned() + key_str)).or(Err(VarError::NotPresent)));
            let mut string = String::new();
            try!(file.read_to_string(&mut string).or(Err(VarError::NotPresent)));
            Ok(string)
        } else {
            Err(VarError::NotPresent)
        }
    } else {
        Err(VarError::NotUnicode(key.as_ref().to_owned()))
    }
}

/// Fetches the environment variable `key` from the current process, returning `None` if the
/// variable isn't set.
pub fn var_os<K: AsRef<OsStr>>(key: K) -> Option<OsString> {
    if let Ok(value) = var(key) {
        Some((value.as_ref() as &OsStr).to_owned())
    } else {
        None
    }
}

/// Sets the environment variable `key` to the value `value` for the current process
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    if let (Some(key_str), Some(value_str)) = (key.as_ref().to_str(), value.as_ref().to_str()) {
        if ! key_str.is_empty() {
            if let Ok(mut file) = File::open(&("env:".to_owned() + key_str)) {
                let _ = file.write_all(value_str.as_bytes());
                let _ = file.set_len(value_str.len() as u64);
            }
        }
    }
}

/// Removes an environment variable from the environment of the current process
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    if let Some(key_str) = key.as_ref().to_str() {
        if ! key_str.is_empty() {
            let _ = fs::remove_file(&("env:".to_owned() + key_str));
        }
    }
}

/// An iterator over the snapshot of the environment variables of this process.
/// This iterator is created through `std::env::vars() and yields (String, String) pairs.`
pub struct Vars {
    vars: Vec<(String, String)>,
    pos: usize
}

impl Iterator for Vars {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let variable = self.vars.get(self.pos);
        self.pos += 1;
        variable.cloned()
    }
}

/// Returns an iterator over the environment variables of the current process
pub fn vars() -> Vars {
    let mut variables: Vec<(String, String)> = Vec::new();
    if let Ok(mut file) = File::open("env:") {
        let mut string = String::new();
        if file.read_to_string(&mut string).is_ok() {
            for line in string.lines() {
                if let Some(equal_sign) = line.chars().position(|c| c == '=') {
                    let name = line.chars().take(equal_sign).collect::<String>();
                    let value = line.chars().skip(equal_sign+1).collect::<String>();
                    variables.push((name, value));
                }
            }
            return Vars { vars: variables, pos: 0 };
        }
    }
    Vars { vars: Vec::new(), pos: 0 }
}
