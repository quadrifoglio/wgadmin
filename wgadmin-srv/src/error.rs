//! Errors that can be returned from the API server.

use std::convert;
use std::io;

use actix_web::HttpResponse;

use serde_json::json;

/// Code for OS error 'permission denied'.
const ERR_PERMISSION_DENIED: i32 = 1;

/// Code for OS error 'file exists'.
const ERR_FILE_EXISTS: i32 = 17;

/// Code for OS error 'no such device'.
const ERR_NOT_FOUND: i32 = 19;

#[derive(Debug, Clone)]
pub enum Error {
    InternalServerError,
    PermissionDenied,
    AlreadyExists,
    NotFound,
    MissingField(&'static str),
    InvalidField(&'static str),
}

impl Error {
    /// Transform the error into an HTTP response that reflects it. Consumes `self`.
    pub fn into_http(self) -> HttpResponse {
        self.into()
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        match err.raw_os_error() {
            Some(code) => match code {
                ERR_PERMISSION_DENIED => Error::PermissionDenied,
                ERR_FILE_EXISTS => Error::AlreadyExists,
                ERR_NOT_FOUND => Error::NotFound,

                _ => Error::InternalServerError,
            },

            None => Error::InternalServerError,
        }
    }
}

impl convert::Into<HttpResponse> for Error {
    fn into(self) -> HttpResponse {
        match self {
            Error::InternalServerError => HttpResponse::InternalServerError().finish(),
            Error::PermissionDenied => HttpResponse::Unauthorized().finish(),
            Error::AlreadyExists => HttpResponse::Conflict().finish(),
            Error::NotFound => HttpResponse::NotFound().finish(),

            Error::MissingField(f) => HttpResponse::BadRequest().json(json! ({
                "error": format!("missing field: {}", f),
            })),

            Error::InvalidField(f) => HttpResponse::BadRequest().json(json! ({
                "error": format!("invalid field: {}", f),
            })),
        }
    }
}
