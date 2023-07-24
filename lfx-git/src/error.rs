// Copyright © 2023 Linear Fox. This file is under GPLv3 License.

use std::{fmt::Display, io};
use git2::{Error, ErrorCode};

pub struct ErrorUtil;

impl ErrorUtil {
    pub fn is_not_found_err(error: &Error) -> bool {
        error.code() == ErrorCode::NotFound
    }

    pub fn is_exists_err(error: &Error) -> bool {
        error.code() == ErrorCode::Exists
    }

    pub fn into_git_err<E: Display>(error: E) -> Error {
        Error::from_str(&error.to_string())
    }

    pub fn into_io_err(error: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    }
}