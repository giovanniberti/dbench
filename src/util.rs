use time::Duration;
use std::fmt::Write;
use std::fmt::{Display, Formatter, Result};

pub trait PrettyPrint {
    fn pretty_print<W: Write>(&self, &mut W);
}

impl PrettyPrint for Duration {
    fn pretty_print<W: Write>(&self, f: &mut W) {
        const NANOS_PER_MS: i64 = 1_000_000; // / any way to refer directly to time::NANOS_PER_MS?

        let nanos = self.num_nanoseconds().unwrap(); // TODO: Better error handling
        write!(f, "{} ms", (nanos as f64) / (NANOS_PER_MS as f64));
    }
}

pub struct PrettyPrinter<T: PrettyPrint>(pub T);

impl<T: PrettyPrint> Display for PrettyPrinter<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut str = String::new();
        self.0.pretty_print(&mut str);
        f.write_str(&str)
    }
}

impl<T: PrettyPrint> From<T> for PrettyPrinter<T> {
    fn from(t: T) -> Self {
        PrettyPrinter(t)
    }
}