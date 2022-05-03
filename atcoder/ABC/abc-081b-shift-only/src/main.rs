// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut a = a as Vec<u32>;
    let mut count = 0;

    loop {
        if a.iter().any(|v| *v % 2 == 1) {
            break;
        }

        a = a.iter().map(|v| *v / 2).collect();
        count += 1;
    }


    println!("{}", count);
}
