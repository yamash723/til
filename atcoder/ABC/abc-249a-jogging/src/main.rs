// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    }

    let takahashi = calc_jogging_distance(a, b, c, x);
    let aoki = calc_jogging_distance(d, e, f, x);

    match takahashi <= aoki {
        true if takahashi == aoki => println!("Draw"),
        true => println!("Aoki"),
        _ => println!("Takahashi"),
    }
}

fn calc_jogging_distance(moving_time: usize, speed: usize, sleep_time: usize, jogging_time: usize) -> usize {
    let step_time = moving_time + sleep_time;
    let step_count = jogging_time / step_time;
    let fraction_time = (jogging_time % step_time).min(moving_time);

    return ((moving_time * step_count) + fraction_time) * speed;
}
