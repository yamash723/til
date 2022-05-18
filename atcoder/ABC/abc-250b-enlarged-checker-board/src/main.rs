// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    for v_n in 0..n {
        for _ in 0..a {
            for h_n in 0..n {
                let c = if (v_n + h_n) % 2 == 0 { '.' } else { '#' };
    
                for _ in 0..b {
                    print!("{}", c);
                }
            }

            print!("\n");
        }
    }
}
