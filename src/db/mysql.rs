use std::error::Error;

use super::database::{BackendError};
use super::connection::ConnectionData;
use super::channel::{DbChannel};

use mysql::{Opts, OptsBuilder, Pool};

pub fn default_port() -> usize {
    3306
}

pub fn connect(params: ConnectionData) -> Result<Box<DbChannel>, BackendError> {
    let builder = {
        let mut tmp = OptsBuilder::new();
        tmp
            .ip_or_hostname(Some(params.host))
            .tcp_port(params.port as u16) // FIXME: Use `num` crate to perform conversion
            .db_name(Some(params.database))
            .user(Some(params.username))
            .pass(params.password);
        tmp
    };

    let opts = Opts::from(builder);
    let pool = Pool::new(opts);

    match pool {
        Ok(p) => Ok(Box::new(p) as Box <DbChannel>),

        Err(e) => {
            println ! ("Unable to connect with MySQL backend.");
            Err(BackendError::IoError(Box::new(e) as Box< Error > ))
        }
    }
}