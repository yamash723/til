//! `ch11_3`クレートはある数値を数値に加える関数を提供する
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, ch11_3::add_two(2));
//! ```

/// この関数は引数に2を加える
///
/// # Examples
///
/// ```
/// use ch11_3::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn internal_adder(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
