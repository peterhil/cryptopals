#![allow(unused_variables)]

extern crate data_encoding;

use data_encoding::HEXLOWER;

pub fn hex_decode(hex: Vec<u8>) -> Vec<u8> {
    return HEXLOWER.decode(&hex).unwrap();
}
