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
    // let source = AutoSource::from("5 1 5");
    input!{
        // from source,
        n: usize,
        x: usize,
        y: usize
    };

    let mut dp = vec![vec![0usize; n+1];n+1];

    for i in 1..n{
        for j in i+1..=n{
            dp[i][j] = j - i
        }
    }

    for i in 1..n{
        for j in i+1..=n{
            dp[i][j] = min(dp[i][j], (x as isize - i as isize).abs() as usize + (y as isize - j as isize).abs() as usize + 1)
        }
    }

    // println!("{:?}", dp);

    let mut bucket = vec![0i64; n+1];

    for i in 1..n{
        for j in i+1..=n{
            bucket[dp[i][j]] += 1;
        }
    }

    for i in 1..n{
        println!("{}", bucket[i]);
    }
}
