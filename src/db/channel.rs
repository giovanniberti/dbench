use std::error::Error;

pub trait DbChannel {
    fn query(&self, &str) -> Result<(), String>;
}
