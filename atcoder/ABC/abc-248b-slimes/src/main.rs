// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    if a >= b {
        println!("0");
        return
    }

    for i in 1..std::u32::MAX {
        if a * (k as usize).pow(i) >= b {
            println!("{:?}", i);
            return
        }
    }
}
