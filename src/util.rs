use time::Duration;
use std::fmt::Write;
use std::fmt::{Display, Formatter, Result};

macro_rules! println_err {
    ($($arg:tt)*) => {
        {
            let res = writeln!(::std::io::stderr(), $($arg)*);
            res.expect("Failed writing to stderr!");
        }
    };
}

macro_rules! expect {
    ($r:expr, $msg:tt) => {
        {
            $r.map_err(|e|{
                println_err!($msg, e.description());
                ::std::process::exit(1);
            }).unwrap()
        }
    };
}

pub fn to_ms_precise(d: &Duration) -> f64 {
    const NANOS_PER_MS: i64 = 1_000_000;

    let nanos = d.num_nanoseconds().unwrap(); // TODO: Better error handling
    (nanos as f64) / (NANOS_PER_MS as f64)
}

pub trait PrettyPrint {
    fn pretty_print<W: Write>(&self, &mut W) -> Result;
}

impl PrettyPrint for Duration {
    fn pretty_print<W: Write>(&self, f: &mut W) -> Result {

        write!(f, "{:.4} ms", to_ms_precise(&self))
    }
}

pub struct PrettyPrinter<T: PrettyPrint>(pub T);

impl<T: PrettyPrint> Display for PrettyPrinter<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = String::new();
        self.0.pretty_print(&mut str).and(f.write_str(&str))
    }
}

impl<T: PrettyPrint> From<T> for PrettyPrinter<T> {
    fn from(t: T) -> Self {
        PrettyPrinter(t)
    }
}