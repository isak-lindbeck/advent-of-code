#[macro_export]
macro_rules! macro_split_parse {
    ($a: expr, $b: expr) => {
        $b.split($a)
        .map(|x| x.parse().unwrap())
        .collect()
    };
}

use std::str::FromStr;

pub fn split_parse<F: FromStr>(sep: &str, input: &str) -> Vec<F> where F::Err: std::fmt::Debug {
    input.split(sep)
        .map(|x| x.parse().unwrap())
        .collect()
}
