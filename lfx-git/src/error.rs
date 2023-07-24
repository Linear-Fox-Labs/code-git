// Copyright © 2023 Linear Fox. This file is under GPLv3 License.

use std::{fmt::Display, io};
use git2::{Error, ErrorCode};

#[derive(Debug)]
pub struct ErrorUtil; 

impl ErrorUtil {
    pub fn is_not_found_err(e: &Error) -> bool {
        e.code() == ErrorCode::NotFound
    }

    pub fn is_exists_err(e: &Error) -> bool {
        e.code() == ErrorCode::Exists
    }

    pub fn into_git_err<E: Display>(e: E) -> Error {
        Error::from_str(&e.to_string())
    }

    pub fn into_io_err(e: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, e.to_string())
    }
}