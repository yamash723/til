extern crate add_one;

fn main() {
    let num = 8;
    println!("Hello world! {} plus one is {}!", num, add_one::add_one(num));
}
