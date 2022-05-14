// -*- coding:utf-8-unix -*-

use std::process::exit;

use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        s: String,
    }

    let unique_chars: Vec<char> = s.chars().unique().collect();
    if unique_chars.iter().count() != s.chars().count() {
        println!("No");
        exit(0)
    }

    let mut exist_upper = false;
    let mut exist_lower = false;
    
    unique_chars.iter().for_each(|c| {
        if !exist_upper {
            exist_upper = c.is_uppercase();
        }
        if !exist_lower {
            exist_lower = c.is_lowercase();
        }

        if exist_upper && exist_lower {
            println!("Yes");
            exit(0)
        }
    });
    
    println!("No");
}
