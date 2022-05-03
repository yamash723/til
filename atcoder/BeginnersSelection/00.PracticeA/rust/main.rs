fn main() {
    let a = read::<u16>();
    let bc = read_vec::<u16>();
    let s = read::<String>();

    let sum = a + &bc[0] + &bc[1];
    println!("{} {}", sum, s);
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
