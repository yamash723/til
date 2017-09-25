# The Rust Programming Language

## Summary

* Rust code uses snake case


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

### Tupl

* Tuple's type don't have to be the same
* Can use type inference

#### Pattern match

```rust
let tup: (i32, f64, u32) = (500, 4.6, 1); // binds to the entire tuple
let (x, y, z) = tup; // pattern match
println!("x:{} y:{} z:{} ", x, y, z);
```

> x:500 y:4.6 z:1

#### Not use Pattern match

```rust
let tup = (500, 4.6, 1); // binds to the entire tuple(type inference)
let x = tup.0; // direct access
let y = tup.1;
let z = tup.2;
println!("x:{} y:{} z:{} ", x, y, z);
```

> x:500 y:4.6 z:1

### Array

* Array have a fixed length
  * cannot grow or shrink in size
    * omg....

``` rust
let array = [0, 1, 2, 3];   // Success
let array = [0, 1, 2, "3"]; // Failed
```

## Function

### Function declare

```rust
// `main` is Entry point
fn main() {
  sample_function1();
  sample_function2(50);

  let result3: i32 = sample_function3(3);
  println!("Sample function 3 : {}", result3);

  let result4: i32 = sample_function4(3);
  println!("Sample function 4 : {}", result4);
}

// No parameter
fn sample_function1() {
  println!("Sample function 1");
}

// Have parameter
fn sample_function2(x: i32) {
  println!("Sample function 2 : {}", x);
}


// Annotate type of return value
fn sample_function3(x: i32) -> i32 {
  // Expressions function
  return x * 5;
}

  // Expressions function(not use `return`)
fn sample_function4(x: i32) -> i32 {
  x * 5
}
```

## Control Flow

### `if`

`if` expressions shouldn't use brackets.

```rust
if number > 5 {
  println!("expression is true")
}
```

`if` is expressions.

```rust
let condition = true;
let number = if condition {
  // shouldn't use semicolon
  5
} else {
  1
};
```

### `loop`

`loop` is infinite loop expressions

```rust
loop {
  println!("Looping!!!!");
  break;
}
```

### `for`

every iteration.

```rust
let list = [10, 20, 30, 40];

for item in list.iter() {
  println!("{}", item);
}
```

can use range.

```rust
for item in 0..5 {
  println!("{}", item);
}

// reverse
for item in (0..5).rev() {
  println!("{}", item);
}
```
