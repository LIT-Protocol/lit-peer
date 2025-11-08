use crate::RecoveryResult;

use path_clean::clean;
use std::{
    ffi::OsString,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

/// This function takes an optional path and returns a concrete Read'er object.
/// This is most useful for command line applications that take either a file
/// or stdin as input. The user can specify "-" or nothing and the result of
/// this function is a Read'er for the stdin stream. If they specify a file,
/// then the Read'er is the file stream. If there is an error opening the file
/// then a crate::error::IoError result.
pub fn reader(path: &Option<PathBuf>) -> RecoveryResult<Box<dyn Read>> {
    match path {
        Some(p) => {
            let pp = clean(p);
            if pp.to_string_lossy() == "-" {
                Ok(Box::new(io::stdin()) as Box<dyn Read>)
            } else {
                Ok(Box::new(File::open(&pp)?) as Box<dyn Read>)
            }
        }
        None => Ok(Box::new(io::stdin()) as Box<dyn Read>),
    }
}

/// This function works in tandem with the above reader function except that
/// it returns a convenient OsString name for the reader. This is used for
/// verbose output to describe where the input is coming from.
#[allow(dead_code)]
pub fn reader_name(path: &Option<PathBuf>) -> RecoveryResult<OsString> {
    match path {
        Some(p) => {
            let pp = clean(p);
            if pp.to_string_lossy() == "-" {
                Ok(OsString::from("stdin"))
            } else {
                Ok(pp.into_os_string())
            }
        }
        None => Ok(OsString::from("stdin")),
    }
}

/// This function works the same as the reader function but is for writers.
/// If the path is provided then the Write'er is for the file stream. If the
/// path is not provided then the Write'er is for the stdout stream.
#[allow(dead_code)]
pub fn writer(path: &Option<PathBuf>) -> RecoveryResult<Box<dyn Write>> {
    match path {
        Some(p) => Ok(Box::new(File::create(clean(p))?) as Box<dyn Write>),
        None => Ok(Box::new(io::stdout()) as Box<dyn Write>),
    }
}

/// This function gives the name for the writer for verbose output purposes.
#[allow(dead_code)]
pub fn writer_name(path: &Option<PathBuf>) -> RecoveryResult<OsString> {
    match path {
        Some(p) => Ok(clean(p).into_os_string()),
        None => Ok(OsString::from("stdout")),
    }
}

/// This function takes an optional path and returns the path if supplied,
/// otherwise it defaults to the current working directory.
#[allow(dead_code)]
pub fn dir(path: &Option<PathBuf>) -> RecoveryResult<PathBuf> {
    match path {
        Some(p) => Ok(clean(p)),
        None => Ok(std::env::current_dir()?),
    }
}

/// This function works with the above dir function but gives the name of the
/// directory for verbose output purposes.
#[allow(dead_code)]
pub fn dir_name(path: &Option<PathBuf>) -> RecoveryResult<OsString> {
    match path {
        Some(p) => Ok(clean(p).into_os_string()),
        None => Ok(OsString::from("pwd")),
    }
}
