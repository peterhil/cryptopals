#![allow(unused_variables)]

pub fn xor_buffers(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    return v1
        .iter()
        .zip(v2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
}

pub fn xor_char(message: &Vec<u8>, chr: u8) -> Vec<u8> {
    return message
        .iter()
        .map(|&letter| letter ^ chr)
        .collect();
}
