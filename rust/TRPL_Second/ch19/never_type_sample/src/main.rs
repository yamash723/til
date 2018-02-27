fn main() {
    let sample: Option<()> = bar();
}

fn bar() -> ! {
    panic!("Err");
}