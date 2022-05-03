// -*- coding:utf-8-unix -*-

use std::collections::{HashSet, HashMap};

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
        s: Chars,
    }

    let xy: Vec<(i32, i32)> = xy;
    let uniq: HashSet<&char> = s.iter().collect();
    if uniq.len() == 1 {
        println!("No");
        return;
    }

    let mut r: HashMap<i32, i32> = HashMap::new();
    let mut l: HashMap<i32, i32> = HashMap::new();

    for i in 0..n as usize {
        let (x, y) = xy[i];
        match s[i] {
            'L' => {
                if let Some(r_x) = r.get(&y) {
                    if *r_x < x {
                        println!("Yes");
                        return;
                    }
                }

                let v = l.entry(y).or_insert(x);
                *v = (*v as i32).max(x);
            },
            'R' => {
                if let Some(l_x) = l.get(&y) {
                    if  x < *l_x {
                        println!("Yes");
                        return;
                    }
                }

                let v = r.entry(y).or_insert(x);
                *v = (*v as i32).min(x);
            },
            _ => (),
        }
    }

    println!("No");
}
