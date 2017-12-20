#[derive(PartialEq, Debug)]
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn create_counter() {
    let counter_by_new = Counter::new();
    let counter_manual = Counter { count: 0 };

    assert_eq!(counter_by_new, counter_manual);
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1)) // (1, 2), (2, 3), (3, 4), (4, 5)
                            .map(|(a, b)| a * b)              // 2, 6, 12, 20
                            .filter(|x| x % 3 == 0)           // 6, 12
                            .sum();                           // 18
    assert_eq!(sum, 18)
}