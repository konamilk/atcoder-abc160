use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        k: i64,
        n: usize,
        a: [i64; n]
    };

    let mut diffs = vec![];

    for i in 0..n-1{
        diffs.push(a[i+1] - a[i])
    }

    diffs.push(k - (a[n-1] - a[0]));

    println!("{}", k - diffs.iter().max().unwrap());
}
