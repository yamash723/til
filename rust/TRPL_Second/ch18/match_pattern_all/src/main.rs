fn main() {
    {
        // 通常のリテラルでのマッチング
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        // 変数を使用したマッチング
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
        // 複数条件
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        // 値の範囲指定
        let x = 5;
        match x {
            1 ... 5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    {
        // Charの範囲指定
        let x = 'c';
        match x {
            'a' ... 'j' => println!("early ASCII letter"),
            'k' ... 'z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    {
        // 構造体を使用したマッチング
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

    {
        // Enumを使用したマッチング
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32,),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            },
            Message::Move { x: x, y: y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y)
            },
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, blue {}", r, g, b)
            }
        }
    }

    {
        // パターンマッチはリファレンスも考慮
        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];

        let sum_of_squares: i32 = points
            .iter()
            .map(|&Point { x: x, y: y }| x * x + y * y )
            .sum();

        println!("Sum value is {}", sum_of_squares);
    }

    {
        // 引数にも「アンダースコアパターン」は可能
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }

        foo(3, 4);
    }

    {
        // タプル内で複数のアンダースコアも可能
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers : {}, {}, {}", first, third, fifth);
            }
        }
    }

    {
        // 変数名の戦闘がアンダーバーの場合はunused warningが発生しない
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {
            println!("found a tring");
        }

        // sは_sにmove済のため、下記はエラー
        // Some(_) = sの場合はmoveしない
        // println!("{:?}", s);
    }

    {
        // タプルの先頭、終端のみのマッチング。中間は..で表現
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            },
        }
    }

    {
        // リファレンスパターン
        // パターンマッチ時にrefやref mutを使用可能
        let robot_name = Some(String::from("Bors"));

        match robot_name {
            Some(ref name) => println!("Found a name: {}", name),
            None => (),
        }

        println!("robot_name is: {:?}", robot_name);
    }

    {
        // ガード節
        // ifで条件を追加可能
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    {
        // 条件指定しつつ、マッチングした値をバインドする
        // { id: id_variable @ 3...7 } と@を使用することで条件指定しつつ値をバインドすることができる
        enum Message {
            Hello { id: i32 }
        }

        let msg = Message::Hello { id: 11 };

        match msg {
            Message::Hello { id: id_variable @ 3...7 } => {
                println!("Found an id in range : {}", id_variable)
            },
            Message::Hello { id: 10...12 } => {
                println!("Found an id in another range")
            },
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            },
        }
    }
}

