// -*- coding:utf-8-unix -*-

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        submissions: [(String, usize);n],
    }

    let n: usize = n;
    let submissions: Vec<(String, usize)> = submissions;

    let mut map = HashSet::new();
    let (mut res_i, mut max_point) = (0, 0);

    for line in submissions.iter().enumerate() {
        let (index, (s, t)) = line;
        if map.contains(s) {
            continue;
        } 

        map.insert(s);

        if max_point >= *t {
            continue;
        }

        res_i = index;
        max_point = *t;

    }

    println!("{}", res_i + 1);
}
