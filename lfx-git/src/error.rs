// Copyright © 2023 Linear Fox. This file is under GPLv3 License.

use std::{fmt::Display, io};
use git2::{Error, ErrorCode};

pub trait ErrorUtils {
    fn is_error_not_found(error: &Error) -> bool;
    fn is_error_exists(error: &Error) -> bool;
    fn convert_to_git_error<E: Display>(error: E) -> Error;
    fn convert_to_io_error(error: Error) -> io::Error;
}

impl ErrorUtils for Error {
    fn is_error_not_found(error: &Error) -> bool {
        error.code() == ErrorCode::NotFound
    }

    fn is_error_exists(error: &Error) -> bool {
        error.code() == ErrorCode::Exists
    }

    fn convert_to_git_error<E: Display>(error: E) -> Error {
        Error::from_str(&error.to_string())
    }

    fn convert_to_io_error(error: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    }
} 