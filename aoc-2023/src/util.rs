use std::str::FromStr;

#[macro_export]
macro_rules! macro_split_parse {
    ($a: expr, $b: expr) => {
        $b.split($a)
        .map(|x| x.parse().unwrap())
        .collect()
    };
}

pub fn split_parse<F: FromStr>(sep: &str, input: &str) -> Vec<F> where F::Err: std::fmt::Debug {
    input.split(sep)
        .map(|x| x.trim().parse().expect(&*format!("Invalid digit '{}'", x)))
        .collect()
}
