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

Matching on Different Errors
-----------------------------------

```rust
fn main() {
    let f = File::open("sample.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            panic!("file is not found: {:?}", err);
        },
        Err(err) => {
            panic!("other error: {:?}", err);
        }
    };
}
```

`ref` is referencing word.

```rust
let a = String::new();

// This b & c is same.
let ref b = a;
let c = &a;
```

`Err(ref err)` is same to `let err = &error`