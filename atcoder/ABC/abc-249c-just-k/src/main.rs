// -*- coding:utf-8-unix -*-

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let s: Vec<String> = s;
    let mut ans = 0;
    
    for i in 0..1 << n {
        let mut map: HashMap<char, usize> = HashMap::new();

        for (j, word) in s.iter().enumerate() {
            if i >> j & 1 == 0 {
                continue;
            }

            for c in word.chars() {
                let char_count = match map.get(&c) {
                    Some(count) => count + 1,
                    _ => 1,
                };
    
                map.insert(c, char_count);
            }
        }

        let pattern = map.iter().filter(|(_, b)| **b == k).count();
        ans = ans.max(pattern);
    }


    println!("{:?}", ans);
}
