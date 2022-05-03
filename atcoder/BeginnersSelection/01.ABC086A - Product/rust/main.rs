fn main() {
    let ab = read_vec::<u16>();
    let (a, b) = (&ab[0], &ab[1]);

    if (a*b)%2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
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

fn read_vec_multiline<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
