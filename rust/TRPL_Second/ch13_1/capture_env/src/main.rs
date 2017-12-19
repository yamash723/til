fn main() {
    let x = vec![1, 2, 3];

    //Fn
    let equal_to_x = |z| z == x;
    println!("{:?}", x); // Is not error. x used by borrow.

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    // FnOnece
    let equal_to_x = move |z| z == x;
    println!("{:?}", x); // Is error. x move to closure.

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
