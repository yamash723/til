// -*- coding:utf-8-unix -*-

use std::{collections::HashMap, ptr, cmp};

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let n: usize = n;
    let q: usize = q;
    let x: Vec<usize> = x;

    let mut balls_val: Vec<usize> = (1..=n).collect();
    let mut balls_pos: Vec<usize> = (0..n).collect();

    for op in x {
        if op > n {
            continue;
        }

        let pos1 = balls_pos[op - 1];
        let pos2 =  if (pos1 + 1) == n {pos1 - 1} else {pos1 + 1};

        let v1 = balls_val[pos1];
        let v2 = balls_val[pos2];
        
        balls_pos.swap(v1 - 1, v2 - 1);
        balls_val.swap(pos1, pos2);
    }

    let ans = balls_val.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", ans);
}