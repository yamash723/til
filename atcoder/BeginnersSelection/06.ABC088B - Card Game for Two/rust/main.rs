fn main() {
    let _max = read::<u32>();
    let mut cards = read_vec::<u32>();

    cards.sort();
    cards.reverse();

    let mut alice = 0;
    let mut bob = 0;
    for (i, card) in cards.iter().enumerate() {
        match i%2 {
            0 => alice += *card,
            _ => bob += *card,
        }
    }

    println!("{}", alice - bob)
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