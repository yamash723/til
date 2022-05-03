// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cards: [u32; n],
    }

    let mut cards: Vec<u32> = cards;
    cards.sort();

    let mut alice = 0;
    let mut bob = 0;

    cards.iter().rev().enumerate().for_each(|(i, v)| {
        if i % 2 == 0 {
            alice += v;
        } else {
            bob += v
        };
    });

    println!("{}", alice - bob);
}
