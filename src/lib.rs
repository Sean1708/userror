//! Some basic functions and macros for printing user-facing errors in command-line programs.
//!
//! # Installation
//!
//! Simply mark userror as a dependency in your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! userror = "0.1.0"
//! ```
//!
//! By default userror prints coloured messages with ansi_term, if you do not want this use
//! `default-features = false`
//!
//! ```toml
//! [dependencies]
//! userror = { version = "0.1.0", default-features = false }
//! ```

#[cfg(feature = "colour")]
extern crate ansi_term;

#[cfg(feature = "colour")]
use ansi_term::Colour;

#[cfg(not(feature = "colour"))]
enum Colour {
    Purple,
    Blue,
    Yellow,
    Red,
}

#[cfg(not(feature = "colour"))]
impl Colour {
    fn paint<'l>(&self, level: &'l str) -> &'l str {
        level
    }
}


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
    () => (concat!(file!(), ":", line!()));

    ($message:expr) => (concat!(file!(), ":", line!(), ": ", $message));

    ($format:expr, $( $val:expr ),+) => (
        format!(concat!(file!(), ":", line!(), ": ", $format), $( $val ),+)
    );
}

/// Prepend file and line info into a call to `.expect()`.
#[macro_export]
macro_rules! expect {
    ($value:expr) => ($value.expect(flm!()));

    ($value:expr, $message:expr) => ($value.expect(flm!($message)));
}

/// Display an internal error message with file and line info.
///
/// Internal errors are bugs or failed invariants in your program, hence file and line info are
/// useful for debugging.
#[macro_export]
macro_rules! internal {
    ($message:expr) => ($crate::internal(flm!($message)));

    ($format:expr, $( $val:expr ),+) => ($crate::internal(&flm!($format, $( $val ),+)));
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
        None => writeln!(io::stderr(), "{}: {}", colour.paint(level), message),
    }
}

/// Print an internal error message.
///
/// Internal errors are bugs or failed invariants in your program. They are not necessarily fatal.
pub fn internal(message: &str) -> io::Result<()> {
    print(Colour::Red, "internal", message)
}

/// Print a fatal error message and panic.
///
/// Fatal errors are errors which can not be recovered from, such as failing to receive user input.
pub fn fatal(message: &str) -> ! {
    print(Colour::Red, "fatal", message).expect("failed to write error message");
    panic!("fatal error occurred");
}

/// Print an error message.
///
/// Errors are recoverable but prevent the program from working properly or in it's entirety, such
/// as failing to open an output file and instead printing results to screen.
pub fn error(message: &str) -> io::Result<()> {
    print(Colour::Red, "error", message)
}

/// Print a warning message.
///
/// Warnings lead to sub-optimal, but not strictly incorrect, behaviour. An example would be
/// failing to load a custom stylesheet and instead using a default one.
pub fn warn(message: &str) -> io::Result<()> {
    print(Colour::Yellow, "warning", message)
}

/// Print some non-erroneous information.
pub fn info(message: &str) -> io::Result<()> {
    print(Colour::Purple, "info", message)
}
