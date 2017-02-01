# dbench
dbench is a simple database query benchmarker written in Rust.
It currently supports MySQL and Postgres. See [Planned features](#planned-features) for more info.

## Table of contents
 - [Install](#install)
 - [Usage](#usage)
 - [Recognized options and flags](#recognized-options-and-flags)
 - [Planned features](#planned-features)
 - [Contribute](#contribute)
 - [License](#license)

## Install
1. Install cargo for your OS/distribution (https://crates.io/install)
2. Run `cargo install dbench`

## Usage
* Run a query 
```
$ dbench -d foo_database -H localhost -u db_user -p -q "select * from bar_table"
```


* Run a query with parametrized URL
```
$ dbench mysql://db_user:db_password@localhost/foo_database -q "select * from bar_table"
```

* Run 10 queries
```
$ dbench mysql://db_user:db_password@localhost/foo_database -q "select * from bar_table" -n 10
```

* Read help
```
$ dbench --help
```

## Recognized options and flags
| Short version | Long version | Accepts parameter |                                Description                              |
|:-------------:|:------------:|:-----------------:|:------------------------------------------------------------------------|
| -d            | --database   | yes               | Database name to which the query is sent                                |
| -H            | --host       | yes               | The host where the database resides                                     |
| -q            | --query      | yes               | The query to be executed by the database                                |
| -j            | --jobs       | yes               | Number of concurrent jobs                                               |
| -u            | --user       | yes               | The username used for authenticating with the database                  |
| -p            | --password   | no                | Flag to indicate whether to use a password or not (asked interactively) |
| -V            | --version    | no                | Prints version information                                              |
| -h            | --help       | no                | Prints help information                                                 |
| -v            | N/A          | no                | Raises verbosity level                                                  |

## Planned features
- ~~PostgreSQL support~~ (Done!)
- Other DBs support (Mongo?)
- More measurements, such as:
  - ~~Req/s (mean)~~ (Done!)
  - ~~Time per request (mean)~~ (Done!)
  - ~~Total time~~ (Done!)
  - Percentage with time (as `ab`)
- ~~Concurrent requests support~~ (Partial, see below)

## Concurrent requests support
Because not all types which implement `DbChannel` are safe to share between different threads
(notably, `postgres::Connection` is not `Sync`), a locking mechanism must be used in order to fire
multiple requests from different threads from the same connection. Thus, concurrency is somewhat limited,
because no thread can fire a request while another is using the connection.

## Contribute
See [CONTRIBUTING.md](../master/CONTRIBUTING.md)

## License
     
[MIT License (c) Giovanni Berti](../master/LICENSE)
