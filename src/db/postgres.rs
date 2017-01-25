use std::borrow::Cow;
use std::error::Error;

use super::database::{BackendError};
use super::connection::ConnectionData;
use super::channel::{DbChannel};

use postgres::{Connection, TlsMode};
use postgres::params::{ConnectParams, ConnectTarget, UserInfo};

pub fn default_port() -> usize {
    5432
}

pub fn connect(params: ConnectionData) -> Result<Box<DbChannel>, BackendError> {
    let conn_params = ConnectParams {
        target: ConnectTarget::Tcp(params.host.into_owned()),
        port: Some(params.port as u16),
        user: Some(UserInfo {
            user: params.username.into_owned(),
            password: params.password.map(Cow::into_owned)
        }),
        database: Some(params.database.into_owned()),
        options: vec![]
    };

    match Connection::connect(conn_params, TlsMode::None) {
        Ok(conn) => Ok(Box::new(conn) as Box<DbChannel>),
        Err(e) => {
            println!("Unable to connect to PostgreSQL backend.");
            Err(BackendError::IoError(Box::new(e) as Box<Error>))
        }
    }
}