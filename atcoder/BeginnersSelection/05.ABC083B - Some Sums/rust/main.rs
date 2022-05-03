fn main() {
    let configure = read_vec::<u32>();

    let max = &configure[0];
    let a = &configure[1];
    let b = &configure[2];
    let mut total_sum = 0;

    for n in 1..(max + 1) {
        let mut j = n;
        let mut sum = 0;

        while j != 0 {
            sum += j % 10;
            j = j / 10;
        }

        if a <= &sum  && &sum <= b {
            total_sum += n;
        }
    }

    println!("{}", total_sum);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}