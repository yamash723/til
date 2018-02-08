enum PatternValue {
    PatternA,
    PatternB,
}

fn main() {
    {
        // match
        let sample = PatternValue::PatternA;
        match sample {
            PatternValue::PatternA => println!("match `PatternA`"),
            PatternValue::PatternB => println!("match `PatternB`"),
        }
    }

    {
        // if let
        let sample = Some("12");
        if let Some(value) = sample {
            println!("sample matched some, value = {}", value);
        }
    }

    {
        // while let
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    {
        // for loop
        let v = vec![1, 2, 3];

        for (index, value) in  v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    {
        // let
        let (x, y, z) = (1, 2, 3);
        println!("x = {}, y = {}, z = {}", x, y, z);
    }

    {
        // function
        let point = (3, 5);
        print_coordinates(&point);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}