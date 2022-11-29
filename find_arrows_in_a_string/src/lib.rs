pub fn arrow_search(string: &str) -> i64 {
    let mut result: i64 = 0;
    let mut negative: bool = false;
    for i in string.chars() {
        if i == '>' {
            result += 1;
        } else if i == '<' {
            result -= 1;
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::arrow_search;
    use std::fmt::Debug;

    fn pretty_assert_eq<T: PartialEq + Debug>(actual: T, expected: T) {
        assert!(actual == expected, "{:?} should equal {:?}", actual, expected)
    }

    #[test]
    fn example_tests() {
        pretty_assert_eq(arrow_search(">."), 1);
        pretty_assert_eq(arrow_search("<--.."), -3);
        pretty_assert_eq(arrow_search("><=><--"), -2);
        pretty_assert_eq(arrow_search(">===>->"), 11);
        pretty_assert_eq(arrow_search("......"), 0);
        pretty_assert_eq(arrow_search("<-->"), 0);
    }
}
