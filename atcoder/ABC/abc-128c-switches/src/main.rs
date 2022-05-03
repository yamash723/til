// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut bulb = Vec::new();

    for _ in 0..m.into() {
        input! {
            k: u32,
            s: [u32; k],
        }

        bulb.push(s.iter().map(|x| *x as u32).collect::<Vec<u32>>());
    }

    input! {
        p: [usize; m],
    }

    // スイッチのON/OFFを全探索
    let success_pattern_count = (0..(1 << n)).filter(|pattern_bit| {
        let mut success = true;
        for bulb_i in 0..m.into() {
            let bulb_on_count = &bulb[bulb_i].iter().filter(|switch| {
                let bit_index = *switch - 1;
                (pattern_bit & 1 << bit_index) == (1 << bit_index)
            }).count();

            if bulb_on_count % 2 != p[bulb_i] {
                success = false;
                break;
            }
        }

        success
    }).count();

    println!("{}", success_pattern_count);
}
