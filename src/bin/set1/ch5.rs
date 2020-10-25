#![warn(clippy::all, rust_2018_idioms)]

use data_encoding::HEXLOWER;

use cryptopals::xor;

fn ch5() -> String {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let encrypted = xor::encrypt_repeated(&plaintext.as_bytes().to_vec(), &key.as_bytes().to_vec());
    let hex = HEXLOWER.encode(&encrypted);
    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    assert_eq!(hex, expected);

    println!("{}", plaintext);
    println!("Key: {}", key);

    return hex;
}

fn main() {
    let hex = ch5();
    println!("Ch5: {:#x?}", &hex);
}
