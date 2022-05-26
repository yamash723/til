// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        mut a: [usize; n],
    }

    let n: usize = n;
    let w: usize = w;
    let mut a: Vec<usize> = a;

    a.sort();

    let mut flag = vec![false; w + 1];

    for p1 in 0..n {
        if a[p1] <= w {
            flag[a[p1]] = true;
        } else {
            break;
        }

        for p2 in (p1 + 1)..n {
            let sum = a[p1] + a[p2];

            if sum <= w {
                flag[sum] = true
            } else {
                break;
            }

            for p3 in (p2 + 1)..n {
                let sum = a[p1] + a[p2] + a[p3];

                if sum <= w {
                    flag[sum] = true
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", flag.iter().filter(|v| **v).count());
}
