fn main() {
    {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is {}", *r1);
            println!("r2 is {}", *r2);
        }
    }

    {
        let address = 0x012345usize;
        let r = address as *const i32;
        println!("{:p}", r);
    }

    {
        unsafe fn dengerous() {}

        unsafe {
            dengerous();
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        split_at_mut(r, 3);

        use std::slice;
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);
            println!("slice address is {:p}", ptr);

            // 対象のスライスのメモリアドレスから直接スライスを作成
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
                )
            }
        }
    }

    {
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }

    }

    {
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
        }
    }
}
