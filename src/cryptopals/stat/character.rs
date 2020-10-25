#![allow(unused_variables)]

use lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;

lazy_static! {
    pub static ref ENGLISH: HashMap<char, f64> = hashmap!{
        ' ' => 0.17857,  // TODO Get more complete statistics for more characters!
        '\'' => 0.02778,
        '.' => 0.01389,
        'a' => 0.08167,
        'b' => 0.01492,
        'c' => 0.02782,
        'd' => 0.04253,
        'e' => 0.12702,
        'f' => 0.02228,
        'g' => 0.02015,
        'h' => 0.06094,
        'i' => 0.06966,
        'j' => 0.00153,
        'k' => 0.00772,
        'l' => 0.04025,
        'm' => 0.02406,
        'n' => 0.06749,
        'o' => 0.07507,
        'p' => 0.01929,
        'q' => 0.00095,
        'r' => 0.05987,
        's' => 0.06327,
        't' => 0.09056,
        'u' => 0.02758,
        'v' => 0.00978,
        'w' => 0.0236,
        'x' => 0.0015,
        'y' => 0.01974,
        'z' => 0.00074,
    };
}

pub fn counts(buf: &String) -> BTreeMap<char, i64> {
    let mut count = BTreeMap::new();

    for c in buf.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    return count;
}

pub fn frequencies(counts: BTreeMap<char, i64>) -> BTreeMap<char, f64> {
    let total: f64 = counts.values().sum::<i64>() as f64;
    let mut frequencies = BTreeMap::new();

    for (ch, count) in &counts {
        frequencies.insert(*ch, *count as f64 / total);
    }

    return frequencies;
}
