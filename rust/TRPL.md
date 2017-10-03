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

## Ownership

At the copy from other variables, this copy run by `Shallow copy` if value stored on heap.
However, in the rust will it run `Move` not `Shallow copy`.

```rust
// allocated on stack. it can copy.
let x = 5;
let y = x;
println!("x: {}, y: {}", x, y);

// same. string literal is allocated on stack
let name_1 = "Big-O";
let name_2 = name_1;
println!("name_1: {}, name_2: {}", name_1, name_2);

// cannot. String::from is allocated on heap.
// variables of allocated on heap. this code is not `shallow copy`, it's the move.
let name_1 = String::from("Big-O");
let name_2 = name_1;

// this code be error at compile. `name_1` is moved. cannot use.
println!("name_1: {}, name_2: {}", name_1, name_2);

// if should be deep copy, use common method of `clone`
let name_1 = String::from("Big-O");
let name_2 = name_1.clone();
println!("name_1: {}, name_2: {}", name_1, name_2);
```

## Borrowing

The same is true when call function with argument.

```rust
fn main() {
  let name = String::from("Unicorn");
  let full_name = add_prefix(name);

  println!("before: {}", name); // this code is error. `name` is already moved.
  println!("after: {}", full_name);
}

fn add_prefix(value: String) -> String {
  format!("{}{}", "Gundam ", value)
}
```

if you call the variables used for call function with argument, add `&` in argument type and variable at call.

```rust
fn main() {
  let name = String::from("Unicorn");
  let full_name = add_prefix(&name);

  println!("before: {}", name); // this code is not error.
  println!("after: {}", full_name);
}

fn add_prefix(value: &String) -> String {
  format!("{}{}", "Gundam ", value)
}
```

## Slice

```rust
fn main() {
  let values = "apple google yahoo github";
  let select = &values[1..11];
  println!("{}", select);
}
```

> pple googl

## Struct

defining structs

```rust
struct User {
  email: String,
  username: String,
  sign_in_count: u64,
  active: bool
}

let mut user = User {
  username: String::from("Yamash"),
  email: String::from("test.yamash@example.com"),
  sign_in_count: 1,
  active: true
};

// access to values by dot notation.
user.active = false;
```

field init

```rust
struct User {
  email: String,
  username: String,
  sign_in_count: u64,
  active: bool
}

fn build_user(username: String, email: String) -> User {
  User {
    // auto matching when the same variables name and struct field.
    username,
    email,
    sign_in_count: 1,
    active: true
  }
}

let username = String::from("Yamash");
let email = String::from("test.yamash@example.com");
let user = build_user(username, email);

println!("{}", user.username);
```

create instance from other instance.

```rust
  let user2 = User {
    email: String::from("test2.yamash@example.com"),
    // copy variables from instance of the `user1`
    ..user1
  };
```

### Implement

```rust
struct Rectangle {
  length u32,
  width u32,
}

impl Rectangle {
  // first argment is `self`
  fn area(&self) -> u32 {
    self.length * self.wodth
  }
}
```