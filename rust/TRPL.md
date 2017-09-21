# The Rust Programming Language

## Variables

### Type

```rust
let variables = 10;       // type inference
let variables: u32 = 10;  // annotate the variables type

```

### Mutable

```rust
let variables = 10;          // no mutable
let mut variables: u32 = 10; // mutable
```

## Tupl

* Tuple's type don't have to be the same
* Can use type inference

### Pattern match

```rust
let tup: (i32, f64, u32) = (500, 4.6, 1); // binds to the entire tuple
let (x, y, z) = tup; // pattern match
println!("x:{} y:{} z:{} ", x, y, z);
```

> x:500 y:4.6 z:1

### Not use Pattern match

```rust
let tup = (500, 4.6, 1); // binds to the entire tuple(type inference)
let x = tup.0; // direct access
let y = tup.1;
let z = tup.2;
println!("x:{} y:{} z:{} ", x, y, z);
```

> x:500 y:4.6 z:1

## Array

* Array have a fixed length
  * cannot grow or shrink in size
    * omg....