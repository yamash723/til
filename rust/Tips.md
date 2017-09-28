Tips for Rust
====================

Concat string
--------------------

```rust
format!("{}{}", "string1", "string2")
```

Iterate with index
---------------------

using normal iterate

```rust
fn main() {
    let mut i = 0;
    let values = ["apple", "google", "facebook"];
    for value in values.iter() {
        println!("{} - {}", i, value);
        i += 1;
    }
}
```

using `enumrate`

```rust
fn main() {
    let values = ["apple", "google", "facebook"];
    for (i, &value) in values.iter().enumerate() {
        println!("{} - {}", i, value);
    }
}
```