// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut v: isize,
        a: isize,
        b: isize,
        c: isize,
    }

    let patterns = [a, b, c];
    
    'main: loop {
        for i in 0..=2 {
            v -= patterns[i];
            
            if v < 0 {
                let label = match i {
                    0 => 'F',
                    1 => 'M',
                    2 => 'T',
                    _ => ' ',
                };
                println!("{}", label);
                break 'main;
            }
        }
    }
}
