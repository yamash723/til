// -*- coding:utf-8-unix -*-

use std::cmp::max;

use proconio::{input, marker::Chars};

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        s: Chars,
    }

    let len = s.len();
    let mut max_count = 0;
    for i in 0..len {
        let mut count = 0;
        for j in i..len {
            if ['A', 'C', 'G', 'T'].contains(&s[j]) {
                count += 1;
            } else {
                break;
            }
        }
        
        max_count = max(max_count, count);
    }

    println!("{}", max_count);
}
