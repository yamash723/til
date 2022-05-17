// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
    }

    let h: usize = h;
    let w: usize = w;
    let r: usize = r;
    let c: usize = c;


    let adjacent_count = |size: usize, pos: usize| {
        match size {
            1 => 0,
            2 => 1,
            _ => if size == pos || pos == 1 {
                1
            } else {
                2
            }
        }
    };
    
    
    println!("{:?}", adjacent_count(h, r) + adjacent_count(w, c));
}
