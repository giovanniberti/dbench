# dbench
**dbench** is a simple database query benchmarker written in Rust.
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

## Example output

```
$ dbench mysql://me:hunter2@localhost/swagdb -q "select * from referents" -n 100 -j 8

Number of requests: 100
Latency per request (mean): 0.1507 ms
Req/ms: 6.635
Total time: 15.0714 ms
Percentage of queries computed within a certain time:
  50%	0.0730 ms
  66%	0.0758 ms
  75%	0.0813 ms
  80%	0.1322 ms
  90%	0.1559 ms
  95%	0.2756 ms
  98%	0.3342 ms
  99%	5.8745 ms
 100%	5.8745 ms (longest request)

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
  - ~~Percentage with time (as `ab`)~~ (Done!)
- ~~Concurrent requests support~~ (Done!)

## Contribute
See [CONTRIBUTING.md](../master/CONTRIBUTING.md)

## License
     
[MIT License (c) Giovanni Berti](../master/LICENSE)
