#![allow(unused_variables)]

pub fn printable(byte: char) -> char {
    if char::is_ascii_control(&byte) {
        return '.';
    } else {
        return byte;
    }
}

pub fn printable_escape(byte: char) -> String {
    if char::is_ascii_control(&byte) {
        return format!("\\{{{}}}", byte as u8);
    } else {
        return byte.to_string();
    }
}

pub fn print(str: Vec<u8>) -> String {
    return str
        .iter()
        .map(|&c| self::printable(c as char))
        .collect();
}
