#![allow(unused_variables)]

pub fn from_vec(buf: Vec<u8>) -> String {
    return match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
}
