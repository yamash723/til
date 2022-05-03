// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    
    let mut count = 0;
    for i in 1u32..=n.into() {
        if i.to_string().chars().count() % 2 == 1 {
            count += 1;
        }
    }

    println!("{}", count);
}
