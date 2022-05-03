fn main() {
    let input_s = read::<String>();

    let mut s = input_s.as_str();
    let words = vec!("dream", "erase", "dreamer", "eraser");

    'outer: loop {
        for word in words.iter() {
            if s.ends_with(word) {
                s = &s[..(s.len() - word.len())];
                continue 'outer;
            }
        }

        break
    }

    match s {
        "" => println!("YES"),
        _ => println!("NO"),
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
