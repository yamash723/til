fn main() {
    let n = read::<u8>();
    let mut a = read_vec::<u32>();
    let mut count = 0;

    loop {
        if to_half(&mut a) {
            count += 1;
        } else {
            break;
        }
    }

    println!("{}", count);
}

fn to_half(a_vec: &mut Vec<u32>) -> bool {
    for a in a_vec.iter_mut() {
        if *a%2 == 1 {
            return false;
        } else {
            *a = *a/2;
        }
    }

    true
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
