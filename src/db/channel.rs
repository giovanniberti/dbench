use std::error::Error;
use mysql::Pool as MySqlPool;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub trait DbChannel: Send + Sync {
    fn query(&self, &str) -> Result<(), String>;
}

impl DbChannel for MySqlPool {
    fn query(&self, query: &str) -> Result<(), String> {
        match self.prep_exec(query, ()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.description().to_string())
        }
    }
}

impl DbChannel for Pool<PostgresConnectionManager> {
    fn query(&self, query: &str) -> Result<(), String> {
        let conn = self.clone().get().unwrap();
        match conn.execute(query, &[]) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.description().to_string())
        }
    }
}