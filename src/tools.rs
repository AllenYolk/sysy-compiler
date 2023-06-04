use std::ops::*;

/// Append a string slice to a mutable reference of another string,
/// starting from a new line.
///
/// If neither of the strings is empty, a new line "\n" is inserted between them.
/// Otherwise, the two strings are concatenated simply.
/// This function is private and is used only in this crate.
pub fn append_line(s: &mut String, l: &str) {
    if !s.is_empty() && !l.is_empty() {
        s.push('\n');
    }
    s.push_str(l);
}

pub fn ceil_to_k<T>(x: T, k: T) -> T
where
    T: Rem<Output = T> + Add<Output = T> + Sub<Output = T> + Copy + PartialEq + From<u8>,
{
    if x % k == 0u8.into() {
        x
    } else {
        x + k - x % k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_line_break_test() {
        let mut s1 = String::from("xx");
        let mut s2 = String::new();
        append_line(&mut s1, "yyy");
        append_line(&mut s2, "yyy");
        assert_eq!(s1, "xx\nyyy");
        assert_eq!(s2, "yyy");
    }
}
