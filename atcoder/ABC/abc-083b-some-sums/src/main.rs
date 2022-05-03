// -*- coding:utf-8-unix -*-

use proconio::input;

fn sum_digits(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0;

    while n > 0 {
        sum += n % 10;
        n = n / 10;
    }

    sum
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut total_sum = 0;
    for i in 1..=n as usize {
        let sum = sum_digits(i);
        if a <= sum && sum <= b {
            total_sum += i;
        }
    }

    println!("{}", total_sum);
}
