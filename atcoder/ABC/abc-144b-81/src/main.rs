// -*- coding:utf-8-unix -*-

use std::process::exit;

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
    }
    
    for i in 1..=9 {
        for j in 1..=9 {
            if n == i * j {
                println!("Yes");
                exit(0);
            }
        }
    }

    println!("No");
}
