// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let nums: Vec<u8> = s.chars().map(|c| c.to_string().parse().unwrap()).collect();

    for i in 0..=9 {
        if !nums.contains(&i) {
            println!("{}", i);
        }
    }
}
