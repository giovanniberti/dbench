#[macro_use]
extern crate mysql;
extern crate clap;
extern crate time;

use std::io::Write;
use time::Duration;

use clap::{Arg, App};
use mysql::{Pool, OptsBuilder, Opts, QueryResult};
use mysql::consts::ColumnType::*;

mod util;
use util::{PrettyPrint, PrettyPrinter};

fn main() {
    let args = App::new("DbBench")
        .version("0.1.0")
        .author("Giovanni Berti <dev.giovanniberti@gmail.com>")
        .about("A database query benchmark program")
        .arg(Arg::with_name("url")
            .value_name("URL")
            .help("The connection url to the db")
            .required(true))
        .arg(Arg::with_name("query")
            .short("q")
            .long("query")
            .value_name("QUERY")
            .help("The query to be executed by the database")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("database")
            .short("d")
            .long("database")
            .value_name("DATABASE")
            .help("The database name")
            .takes_value(true))
        .arg(Arg::with_name("username")
            .short("u")
            .long("user")
            .value_name("USER")
            .help("The username used for authenticating with the database")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .value_name("PASSWORD")
            .help("The password used for authenticating with the database")
            .takes_value(true))
        .get_matches();


    let url = args.value_of("url");
    let username = args.value_of("username");
    let password = args.value_of("password");
    let query = args.value_of("query");
    let db = args.value_of("database");

    // MYSQL CONNECT //
    /*
    let opts = OptsBuilder::new()
        .ip_or_hostname(url)
        .user(username)
        .pass(password)
        .db_name(db);
     */

    let url_opts = Opts::from_url(url.unwrap()).unwrap();
    let pool = mysql::Pool::new(url_opts).map_err(|e| {
        writeln!(std::io::stderr(), "Error while trying to connect to database: {}", e);
        std::process::exit(1);
    }).unwrap();

    let duration = Duration::span(|| {
        pool.prep_exec(query.unwrap(), ());
    });

    println!("Query took: {}", PrettyPrinter::from(duration));
}