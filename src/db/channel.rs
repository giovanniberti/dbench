use std::error::Error;
use mysql::Pool;
use postgres::Connection;

pub trait DbChannel {
    fn query(&self, &str) -> Result<(), String>;
}

impl DbChannel for Pool {
    fn query(&self, query: &str) -> Result<(), String> {
        match self.prep_exec(query, ()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.description().to_string())
        }
    }
}

impl DbChannel for Connection {
    fn query(&self, query: &str) -> Result<(), String> {
        match self.execute(query, &[]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.description().to_string())
        }
    }
}