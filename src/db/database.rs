use std::error::Error;
use std::fmt;
use std::fmt::{Display, Debug, Formatter};
use db::channel::DbChannel;
use db::connection::ConnectionData;



pub trait Backend: Debug {
    fn default_port(&self) -> usize;
    fn connect(&self, ConnectionData) -> Result<Box<DbChannel>, BackendError>;
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