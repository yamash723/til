fn main() {
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = 5;
        match x {
            1 ... 5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    {
        let x = 'c';
        match x {
            'a' ... 'j' => println!("early ASCII letter"),
            'k' ... 'z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        // a, bに代入
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            // 構造体のyが0の場合にマッチ
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            // 構造体のxが0の場合にマッチ
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
}

