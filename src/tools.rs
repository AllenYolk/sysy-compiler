pub fn append_line(s: &mut String, l: &str) {
    if !s.is_empty() {
        s.push('\n');
    }
    s.push_str(l);
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
