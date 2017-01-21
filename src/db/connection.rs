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

pub struct Connection {}