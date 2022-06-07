// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
        b: [usize; k],
    }

    let a: Vec<usize> = a;
    let b: Vec<usize> = b;

    let max_a = a.iter().max().unwrap();
    for pos in a.iter().enumerate().filter(|(_, &v)| v == *max_a).map(|(pos, _)| pos) {
        if b.iter().any(|v| pos + 1 == *v) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
