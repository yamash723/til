struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position { x: x, y: y }
    }

    pub fn distance(&self, pos: &Position) -> u32 {
        (self.x as i32 - pos.x as i32).abs() as u32 +
        (self.y as i32 - pos.y as i32).abs() as u32
    }

    pub fn can_move(&self, time: u32, pos: &Position) -> bool {
        let t = self.distance(pos);
        if t == 0 || t > time {
            false
        } else {
            t%2 == time%2
        }
    }
}

fn main() {
    let n = read::<u32>();
    let ts = read_vec_multiline::<u32>(n);

    let mut current_pos = Position::new(0, 0);
    let mut current_time = 0;

    for t in ts {
        let time = t[0];
        let pos = Position::new(t[1], t[2]);

        if !current_pos.can_move(time - current_time, &pos) {
            println!("No");
            return;
        }

        current_pos = pos;
        current_time = time;
    }

    println!("Yes");
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec_multiline<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_move_test() {
        let pos = Position::new(0, 0);
        assert!(!pos.can_move(1, &Position::new(0, 0)));

        // short range
        assert!(!pos.can_move(2, &Position::new(0, 0)));
        assert!(!pos.can_move(2, &Position::new(1, 0)));
        assert!(!pos.can_move(2, &Position::new(0, 1)));

        // same range
        assert!( pos.can_move(10, &Position::new(3, 7)));
        assert!( pos.can_move(10, &Position::new(5, 5)));
        assert!(!pos.can_move(10, &Position::new(6, 5)));

        // over range - odd
        assert!(pos.can_move(17, &Position::new(3, 8)));
        assert!(pos.can_move(17, &Position::new(5, 6)));

        // over range - even
        assert!(pos.can_move(18, &Position::new(3, 9)));
        assert!(pos.can_move(20, &Position::new(5, 9)));
    }

    #[test]
    fn distance_test() {
        let pos = Position::new(5, 2);
        assert_eq!(pos.distance(&Position::new(0, 0)), 7);
        assert_eq!(pos.distance(&Position::new(6, 0)), 3);
        assert_eq!(pos.distance(&Position::new(2, 0)), 5);

        let pos = Position::new(0, 0);
        assert_eq!(pos.distance(&Position::new(2, 0)), 2);
    }
}