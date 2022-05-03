use std::collections::HashSet;

fn main() {
    let n = read::<u32>();
    let rice_cakes = read_multiline::<u32>(n);
    let rice_cakes: HashSet<u32> = rice_cakes.into_iter().collect();

    println!("{}", rice_cakes.len());
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_multiline<T: std::str::FromStr>(n: u32) -> Vec<T> {
    (0..n).map(|_| read()).collect()
}