fn main() {
    let a = read::<u32>();
    let b = read::<u32>();
    let c = read::<u32>();
    let x = read::<u32>();

    let mut count = 0;
    for ia in 0..(a + 1) {
        for ib in 0..(b + 1) {
            for ic in 0..(c + 1) {
                if (ia*500)+(ib*100)+(ic*50) == x {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
