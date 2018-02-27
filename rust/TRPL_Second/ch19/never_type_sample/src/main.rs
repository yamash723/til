fn main() {
    // 型強制することができる
    let sample: Option<()> = bar();
}

fn bar() -> ! {
    panic!("Err");
}