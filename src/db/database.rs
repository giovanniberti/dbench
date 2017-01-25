use std::error::Error;
use std::fmt;
use std::fmt::{Display, Debug, Formatter};
use db::channel::DbChannel;
use db::connection::ConnectionData;

use db::mysql;
use db::postgres;

#[derive(Debug)]
pub enum Database {
    MySQL,
    Postgres
}

pub trait Backend: Debug {
    fn default_port(&self) -> usize;
    fn connect(&self, ConnectionData) -> Result<Box<DbChannel>, BackendError>;
}

// FIXME: Find a way to statically dispatch
impl Backend for Database {
    fn default_port(&self) -> usize {
        match *self {
            Database::MySQL => mysql::default_port(),
            Database::Postgres => postgres::default_port()
        }
    }

    fn connect(&self, cdata: ConnectionData) -> Result<Box<DbChannel>, BackendError> {
        match *self {
            Database::MySQL => mysql::connect(cdata),
            Database::Postgres => postgres::connect(cdata)
        }
    }
}

#[derive(Debug)]
pub enum BackendError {
    IoError(Box<Error>)
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for BackendError {
    fn description(&self) -> &str {
        match *self {
            BackendError::IoError(_) => "Error estabilishing a database connection!"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BackendError::IoError(ref err) => Some(&**err)
        }
    }
}