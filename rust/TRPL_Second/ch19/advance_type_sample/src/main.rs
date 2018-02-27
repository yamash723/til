fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    {
        type Thunk = Box<Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {
            unimplemented!();
        }

        fn returns_long_type() -> Thunk {
            unimplemented!();
        }
    }

    {
        use std::io::Error;
        use std::fmt;

        type Result_<T> = Result<T, std::io::Error>;

        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result_<usize>;
            fn flush(&mut self) -> Result<(), Error>;

            fn write_all(&mut self, buf: &[u8]) -> Result_<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result_<()>;
        }
    }
}
