extern crate regex;

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use db::database::{Backend, BackendError, Database};
use db::channel::DbChannel;
use self::regex::RegexBuilder;

#[derive(Debug)]
pub struct ConnectionParams<'c, B> where B: Backend {
    pub data: ConnectionData<'c>,
    pub backend: B
}

#[derive(Debug)]
pub struct ConnectionData<'c> {
    pub host: Cow<'c, str>,
    pub port: usize,
    pub database: Cow<'c, str>,
    pub username: Cow<'c, str>,
    pub password: Option<Cow<'c, str>>,
}

#[derive(Debug)]
pub enum ParamsError {
    UnsupportedBackendError,
    MalformedURLError
}

impl Display for ParamsError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for ParamsError {
    fn description(&self) -> &str {
        match *self {
            ParamsError::UnsupportedBackendError => "Unsupported backend!",
            ParamsError::MalformedURLError => "Error while trying to parse URL!"
        }
    }
}

pub trait IntoConnectionParams<'c> {
    fn into(self) -> Result<ConnectionParams<'c, Database>, ParamsError>;
}

impl<'c> IntoConnectionParams<'c> for ConnectionParams<'c, Database> {
    fn into(self) -> Result<ConnectionParams<'c, Database>, ParamsError> {
        Ok(self)
    }
}

pub struct Connection {}

#[derive(Debug)]
pub enum ConnectionError {
    ParamsError(ParamsError),
    BackendError(BackendError)
}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for ConnectionError {
    fn description(&self) -> &str {
        match *self {
            ConnectionError::ParamsError(_) => "Wrong parameters!",
            ConnectionError::BackendError(_) => "Backend error! {}"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::ParamsError(ref e) => Some(*&e),
            ConnectionError::BackendError(ref e) => Some(*&e)
        }
    }
}