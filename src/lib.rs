extern crate ansi_term;

use ansi_term::Colour;
use std::io::{self, Write};

/// Prepend file and line info into a given message.
///
/// This is useful for internal errors to avoid messy uses of `file!` and `line!`.
///
/// The first argument to this macro must always be a string literal. If a single argument is
/// given then a `&'static str` will be returned. If multiple arguments are given the the first
/// argument is used as a format string for `format!`.
#[macro_export]
macro_rules! flm {
    ($message:expr) => ({ concat!(file!(), ":", line!(), ": ", $message) });

    ($format:expr, $( $val:expr ),+) => ({
        format!(concat!(file!(), ":", line!(), ": ", $format), $( $val ),+)
    });
}

#[macro_export]
macro_rules! internal {
    ($message:expr) => (print(Colour::Red, "internal", flm!($message)));

    ($format:expr, $( $val:expr ),+) => (print(Colour::Red, "internal", &flm!($format, $( $val ),+)));
}


fn print(colour: Colour, level: &str, message: &str) -> io::Result<()> {
    let program = try!(std::env::current_exe());
    let program = program.file_name().and_then(|n| n.to_str());
    match program {
        Some(name) => writeln!(
            io::stderr(),
            "{}:{}: {}",
            Colour::Blue.paint(name),
            colour.paint(level),
            message,
        ),
        None => writeln!(io::stderr(), "{} {}", colour.paint(level), message),
    }
}

pub fn fatal(message: &str) -> ! {
    print(Colour::Red, "fatal", message).expect("failed to write error message");
    panic!("fatal error occurred");
}

pub fn error(message: &str) -> io::Result<()> {
    print(Colour::Red, "error", message)
}

pub fn warn(message: &str) -> io::Result<()> {
    print(Colour::Yellow, "warning", message)
}

pub fn info(message: &str) -> io::Result<()> {
    print(Colour::Purple, "info", message)
}
