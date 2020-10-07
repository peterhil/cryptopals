// https://rosettacode.org/wiki/Letter_frequency#Rust

use std::fs::File;
use std::io::{Read};
use std::{env};

use cryptopals::stat::character;
use cryptopals::io::{exit_err};

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

fn main() {
    char_statistics();
}
