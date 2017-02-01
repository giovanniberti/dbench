#[macro_use]
extern crate mysql;
#[macro_use]
extern crate lazy_static;
extern crate postgres;
extern crate clap;
extern crate time;
extern crate rpassword;
extern crate num;
extern crate rayon;

#[macro_use]
mod util;
mod db;

use std::borrow::Cow;
use std::io::Write;
use std::error::Error;
use std::sync::{Arc, Mutex};
use time::Duration;
use num::cast::ToPrimitive;
use clap::{Arg, App};
use util::PrettyPrinter;
use db::connection::{Connection, ConnectionData, ConnectionParams};
use db::database::{Backend, Database};
use db::channel::DbChannel;
use rayon::prelude::*;
use rayon::Configuration;

fn main() {
    let args = App::new("DbBench")
        .version("0.1.0")
        .author("Giovanni Berti <dev.giovanniberti@gmail.com>")
        .about("A database query benchmark program")
        .arg(Arg::with_name("url")
            .value_name("URL")
            .help("The connection url to the db. If you specify the URL, you must not pass host, backend, username and password arguments.")
            .conflicts_with("manual"))
        .arg(Arg::with_name("query")
            .short("q")
            .long("query")
            .value_name("QUERY")
            .help("The query to be executed by the database")
            .required(true))
        .arg(Arg::with_name("host")
            .short("H")
            .long("host")
            .value_name("HOST")
            .help("The host where the database resides")
            .group("manual"))
        .arg(Arg::with_name("database")
            .short("d")
            .long("database")
            .value_name("DATABASE")
            .help("The database name")
            .group("manual"))
        .arg(Arg::with_name("username")
            .short("u")
            .long("user")
            .value_name("USER")
            .help("The username used for authenticating with the database")
            .group("manual"))
        .arg(Arg::with_name("backend")
            .short("b")
            .long("backend")
            .value_name("BACKEND")
            .help("The database backend used")
            .possible_values(&["mysql", "postgres"])
            .group("manual"))
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .help("Flag to indicate whether to use a password or not (asked interactively)"))
        .arg(Arg::with_name("requests")
            .short("n")
            .value_name("NUMBER")
            .help("The number of query requests to send to the database"))
        .arg(Arg::with_name("jobs")
            .short("j")
            .long("jobs")
            .value_name("JOBS")
            .help("The number of concurrent jobs that will query the database"))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .help("Verbosity level"))
        .get_matches();

    let chan_raw = {
        let chan_inner = {
            if args.is_present("url") {
                let url = args.value_of("url").unwrap();
                Connection::connect(url)
            } else {
                let backend = args.value_of("backend");
                let db = args.value_of("database");
                let username = args.value_of("username");
                let host = args.value_of("host");
                let port = args.value_of("port");
                let password = args.value_of("password");

                // db, username, host and backend must ALL be present
                match (db, username, host, backend) {
                    (Some(db), Some(username), Some(host), Some(backend_str)) => {
                        let database = match backend_str {
                            "mysql" => Database::MySQL,
                            "postgres" => Database::Postgres,
                            _ => unreachable!()
                        };

                        Connection::connect(ConnectionParams {
                            data: ConnectionData {
                                host: Cow::from(host),
                                port: port.and_then(|s| str::parse::<usize>(s).ok()).unwrap_or(database.default_port()),
                                database: Cow::from(db),
                                username: Cow::from(username),
                                password: password.map(Cow::from)
                            },
                            backend: database
                        })
                    },
                    _ => {
                        println_err!("Missing parameters: ensure that parametrized URL or db, username, host and backend are present.");
                        std::process::exit(1);
                    }
                }
            }
        };

        match chan_inner {
            Ok(c) => c,
            Err(e) => {
                println_err!("Error while trying to create connection to db! {}", e.description());
                std::process::exit(1);
            }
        }
    };

    let jobs = args.value_of("jobs").map(str::parse::<usize>).and_then(Result::ok).unwrap_or(1);
    let query = args.value_of("query");
    let verbosity = args.occurrences_of("verbosity");

    let times = args.value_of("requests").map(str::parse::<usize>).and_then(|r| {
        r.map_err(|_| println_err!("Invalid argument passed to `-n` flag. Defaulting to 1")).ok()
    }).unwrap_or(1);

    let measure = |chan: &Box<DbChannel>| {
        let duration = Duration::span(|| {
            match chan.query(query.unwrap()) {
                Ok(_)  => (),
                Err(cause) => println!("Error: {}", cause)
            }
        });

        if verbosity > 0 {
            println!("Query took: {}", PrettyPrinter::from(duration));
        }

        duration
    };

    let mut durations = Vec::with_capacity(times);

    if jobs > 1 {
        let chan = Arc::new(Mutex::new(chan_raw));

        let config = Configuration::new().set_num_threads(jobs);

        match rayon::initialize(config) {
            Ok(()) => (),
            Err(e) => println!("Error while initializing rayon: {}", e)
        }

        (0..times).into_par_iter().map(|_| {
            let chan = chan.lock().unwrap();
            measure(&*chan)
        }).collect_into(&mut durations);
    } else {
        let chan = chan_raw;
        for _ in 0..times {
            durations.push(measure(&chan));
        }
    }

    let sum = durations.iter().fold(Duration::zero(), |d, &c| d + c);
    println!("Number of requests: {}", times);
    println!("Latency per request (mean): {}", PrettyPrinter::from(sum / times.to_i32().unwrap()));
    println!("Req/ms: {0:.3}", times.to_f64().unwrap() / util::to_ms_precise(&sum));
    println!("Total time: {}", PrettyPrinter::from(sum));
}