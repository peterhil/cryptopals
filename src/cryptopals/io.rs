#![allow(unused_variables)]

use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Lines, Read, Result, Write};
use std::path::Path;
use std::process;

#[inline]
pub fn exit_err<T>(msg: T, code: i32) -> ! where T: Display {
    writeln!(&mut io::stderr(), "{}", msg).expect("Could not write to stderr");
    process::exit(code)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Reuse the same String buffer
// Ref: https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
//
// TODO: Maybe return Vec<Result<Vec<T>>> in order to be able to
// defer error handling to caller and use more functional style?
pub fn read_lines_by<T>(file_name: &str, func: fn(&str) -> Vec<T>) -> Result<Vec<T>> {
    let file = File::open(&file_name)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut contents = vec![];

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                // EOF: save last file address to restart from this address for next run
                if bytes_read == 0 { break }

                let mut result = func(&line.trim());
                contents.append(&mut result);

                // Do not accumulate data
                line.clear();
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    return Ok(contents);
}
