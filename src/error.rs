use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "std::io Error")]
    StdIoError,
    #[fail(display = "Kubernetes request error")]
    RequestError,
    #[fail(display = "Kubernetes response error")]
    ResponseError,
    #[fail(display = "reqwest error")]
    ReqwestError,
    #[fail(display = "Unexpected method is used")]
    UnexpectedMethod,
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    #[allow(dead_code)]
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<k8s_openapi::RequestError> for Error {
    fn from(error: k8s_openapi::RequestError) -> Error {
        Error {
            inner: error.context(ErrorKind::RequestError),
        }
    }
}

impl From<k8s_openapi::ResponseError> for Error {
    fn from(error: k8s_openapi::ResponseError) -> Error {
        Error {
            inner: error.context(ErrorKind::ResponseError),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::ReqwestError),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::StdIoError),
        }
    }
}
