enum List {
    // is error, recursive type.
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

use List::{ Cons, Nil };

fn main() {
    // is error
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
