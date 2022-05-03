fn main() {
    let conditons = read_vec::<i32>();
    let n = &conditons[0];
    let sum = &conditons[1];

    let mut ans = None;
    for x in 0..(n + 1) {
        for y in 0..(n - x + 1) {
            let z = n - x - y;

            if (x * 10000) + (y * 5000) + (z * 1000) == *sum {
                ans = Some((x, y, z));
                break;
            }
        }
    }

    let (x, y, z) = ans.unwrap_or((-1, -1, -1));
    println!("{} {} {}", x, y, z);
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
