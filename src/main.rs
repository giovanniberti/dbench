#[macro_use]
extern crate mysql;
extern crate clap;
extern crate time;
extern crate rpassword;
extern crate num;

use std::io::Write;
use std::error::Error;
use time::Duration;

use mysql::{Opts, OptsBuilder};

use num::cast::ToPrimitive;

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
            .help("The username used for authenticating with the database"))
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .help("Flag to indicate whether to use a password or not (asked interactively)"))
        .arg(Arg::with_name("requests")
            .short("n")
            .value_name("NUMBER")
            .help("The number of query requests to send to the database"))
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

        if args.is_present("password") {
            let pass = rpassword::prompt_password_stdout("Password: ").ok();
            tmp.pass(pass);
        }

        tmp
    };

    let opts = Opts::from(builder);
    let pool = expect!(mysql::Pool::new(opts), "Error while trying to connect to database: {}");
    let query = args.value_of("query");

    let times = args.value_of("requests").map(str::parse::<usize>).and_then(|r| {
        r.map_err(|_| println_err!("Invalid argument passed to `-n` flag. Defaulting to 1")).ok()
    }).unwrap_or(1);

    let mut durations = Vec::with_capacity(times);
    for _ in 0..times {
        let duration = Duration::span(|| {
            expect!(pool.prep_exec(query.unwrap(), ()), "Error while executing query: {}");
        });

        println!("Query took: {}", PrettyPrinter::from(duration));
        durations.push(duration);
    }

    let sum = durations.iter().fold(Duration::zero(), |d, &c| d + c);
    println!("Latency per request (mean): {}", PrettyPrinter::from(sum / times.to_i32().unwrap()));
}