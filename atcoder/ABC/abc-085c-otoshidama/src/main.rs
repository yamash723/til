// -*- coding:utf-8-unix -*-

use std::process::exit;

use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
    }

    let n: usize = n;
    let y: usize = y;
    
    for ix in 0..=n {
        for iy in 0..=n - ix {
            let iz = n - ix - iy;
            if y == ix * 10000 + iy * 5000 + iz * 1000 {
                println!("{} {} {}", ix, iy, iz);
                exit(0);
            }
        }
    }

    println!("-1 -1 -1");
}
