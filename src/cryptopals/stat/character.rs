#![allow(unused_variables)]

use std::collections::btree_map::BTreeMap;

pub fn counts(buf: String) -> BTreeMap<char, i64> {
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
