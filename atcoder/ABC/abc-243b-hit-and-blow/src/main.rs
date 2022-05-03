// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    let a = a as Vec<i32>;
    let b = b as Vec<i32>;

    println!("{}", (0..n as usize).filter(|i| a[*i] == b[*i]).count());

    println!("{}", a.iter().enumerate().filter(|(i, va)| {
        let pos = b.iter().position(|vb| *va == vb);

        match pos {
            Some(position) => position != *i,
            None => false,
        }
    }).count());
}
