// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    match &s.len() {
        1 => println!("{}", (0..6).map(|_| s.to_string()).collect::<Vec<String>>().join("")),
        2 => println!("{}", (0..3).map(|_| s.to_string()).collect::<Vec<String>>().join("")),
        3 => println!("{}", (0..2).map(|_| s.to_string()).collect::<Vec<String>>().join("")),
        _ => panic!("")
    }
}
