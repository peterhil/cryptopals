#![allow(unused_variables)]

use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process;

#[inline]
pub fn exit_err<T>(msg: T, code: i32) -> ! where T: Display {
    writeln!(&mut io::stderr(), "{}", msg).expect("Could not write to stderr");
    process::exit(code)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
