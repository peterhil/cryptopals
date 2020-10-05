#![allow(unused_variables)]

pub fn printable(byte: char) -> char {
    if char::is_ascii_graphic(&byte) {
        return byte;
    } else {
        return '.';
    }
}

pub fn printable_escape(byte: char) -> String {
    if char::is_ascii_graphic(&byte) {
        return byte.to_string();
    } else {
        return format!("\\{{{}}}", byte as u8);
    }
}

pub fn print(str: Vec<u8>) -> String {
    return str
        .iter()
        .map(|&c| self::printable(c as char))
        .collect();
}
