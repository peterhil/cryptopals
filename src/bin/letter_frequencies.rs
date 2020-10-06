// https://rosettacode.org/wiki/Letter_frequency#Rust

use std::{env, process};
use std::io::{self, Read, Write};
use std::fmt::Display;
use std::fs::File;

use cryptopals::stat::character;

fn char_statistics() {
    let filename = env::args().nth(1)
        .ok_or("Please supply a file name")
        .unwrap_or_else(|e| exit_err(e, 1));

    let mut buf = String::new();

    File::open(&filename)
        .unwrap_or_else(|e| exit_err(e, 2))
        .read_to_string(&mut buf)
        .unwrap_or_else(|e| exit_err(e, 3));

    let count = character::counts(&buf);

    println!("Number of occurences per character");
    for (ch, count) in &count {
        println!("{:?}: {}", ch, count);
    }

    let frequencies = character::frequencies(count);
    println!("Number of occurences per character");
    for (ch, freq) in &frequencies {
        println!("{:?}: {}", ch, freq);
    }
}

#[inline]
fn exit_err<T>(msg: T, code: i32) -> ! where T: Display {
    writeln!(&mut io::stderr(), "{}", msg).expect("Could not write to stderr");
    process::exit(code)
}

fn main() {
    char_statistics();
}
