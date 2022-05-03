// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut count = 0;

    for i in 1..=n.into() {
        if i % 2 == 0 {
            continue;
        }

        let mut c = 0;
        for j in 1..=i {
            if i % j == 0 {
                c += 1;
            } 
        }

        if c == 8 {
            count += 1;
        }

    }

    println!("{}", count);
}
