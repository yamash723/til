// ----------------------------------------------------------
// First Tests
// ----------------------------------------------------------

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    // 一つ上の階層(Rectangeなど)をインポート
    // そうすることによって直接「Rectangle」を使用できる
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 5, width: 3 };
        let smaller = Rectangle { length: 3, width: 2 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 5, width: 3 };
        let smaller = Rectangle { length: 3, width: 2 };

        assert!(!smaller.can_hold(&larger));
    }
}

// ----------------------------------------------------------
// Second Tests
// ----------------------------------------------------------

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests_second {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// ----------------------------------------------------------
// Third Tests
// ----------------------------------------------------------

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests_third {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        // 第2引数にエラー時の表示文言を入力可能
        // 第3引数以降は文言用変数
        assert!(
            result.contains("Carol"),
            "Greeting did not conatin name, value was {}",
            result
            );
    }
}

// ----------------------------------------------------------
// Four Tests
// ----------------------------------------------------------

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests_four {
    use super::*;

    #[test]
    fn equal_1() {
        let guess = Guess::new(1);
        assert_eq!(guess.value, 1);
    }

    #[test]
    fn equal_100() {
        let guess = Guess::new(100);
        assert_eq!(guess.value, 100);
    }

    #[test]
    fn greater_than_1() {
        let guess = Guess::new(5);
        assert_eq!(guess.value, 5);
    }

    #[test]
    fn less_than_100() {
        let guess = Guess::new(95);
        assert_eq!(guess.value, 95);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1, got 0.")]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }

}