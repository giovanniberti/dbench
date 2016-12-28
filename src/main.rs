#[macro_use]
extern crate mysql;
extern crate clap;
extern crate time;

use std::io::Write;
use std::error::Error;
use time::Duration;

use mysql::{Opts, OptsBuilder};

use clap::{Arg, App};

#[macro_use]
mod util;
use util::PrettyPrinter;

fn main() {
    let args = App::new("DbBench")
        .version("0.1.0")
        .author("Giovanni Berti <dev.giovanniberti@gmail.com>")
        .about("A database query benchmark program")
        .arg(Arg::with_name("url")
            .value_name("URL")
            .help("The connection url to the db"))
        .arg(Arg::with_name("host")
            .short("H")
            .long("host")
            .value_name("HOST")
            .help("The host where the database resides"))
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

    let builder = {
        let mut tmp = OptsBuilder::new();

        if args.is_present("url") {
            let url = args.value_of("url");
            let url_opts = expect!(Opts::from_url(url.unwrap()), "Failed to parse provided URL! {}");

            tmp = OptsBuilder::from_opts(url_opts);

            // No need to fetch port, username, db
        } else {
            let db = args.value_of("database");
            let username = args.value_of("username");
            let host = args.value_of("host");

            // db, username and host must ALL be present
            match (db, username, host) {
                (Some(_), Some(_), Some(_)) => (),
                _ => {
                    println_err!("Missing parameters: ensure that parametrized URL or db, username and host are present.");
                    std::process::exit(1);
                }
            }

            tmp
                .db_name(db)
                .user(username)
                .ip_or_hostname(host);
        }

        args.value_of("password").map(|p| tmp.pass(Some(p)));

        tmp
    };

    let opts = Opts::from(builder);

    let pool = mysql::Pool::new(opts).map_err(|e| {
        println_err!("Error while trying to connect to database: {}", e.description());
        std::process::exit(1);
    }).unwrap();

    let query = args.value_of("query");

    let duration = Duration::span(|| {
        pool.prep_exec(query.unwrap(), ()).map_err(|e| {
            println_err!("Error while executing query: {}", e.description());
            std::process::exit(1);
        }).unwrap();
    });

    println!("Query took: {}", PrettyPrinter::from(duration));
}