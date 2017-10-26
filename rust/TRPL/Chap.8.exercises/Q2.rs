fn main() {
    let from_str = String::from("apple");
    let first_char = &from_str.chars().next().unwrap();
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    let to_str: String;

    if vowels.contains(&first_char) {
        to_str = format!("{}-hay", &from_str);
    } else {
        to_str = format!("{}-{}ay", &from_str[1..], &first_char);
    }

    println!("{} -> {}", from_str, &to_str);
}