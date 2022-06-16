// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut count: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    for line in s.iter() {
        for (i, char) in line.chars().enumerate() {
            let reel_number = char.to_digit(10).unwrap() as usize;
            count[reel_number][i] += 1;
        }
    }

    let mut ans = std::usize::MAX;

    for reel_number in 0..10 {
        let mut max = 0;
        for i in 0..10 {
            if count[reel_number][i] == 0 {
                continue;
            }

            max = max.max((count[reel_number][i].saturating_sub(1) * 10) + i);
        }
        ans = ans.min(max);
    }

    println!("{}", ans);
}
