// -*- coding:utf-8-unix -*-

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u32; n],
    }

    let d: HashSet<u32> = d.into_iter().collect();

    println!("{}", d.len());
}
