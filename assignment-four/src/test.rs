#[cfg(test)]
mod tests {
    use crate::duplicate::duplicate_finder;
    use crate::palindrome::is_palindrome;
    use crate::rotation::is_rotation;
    #[test]
    fn odd_length_palindrome() {
        assert!(is_palindrome("amama", 0, 4));
    }
    #[test]
    fn even_length_palindrome() {
        assert!(is_palindrome("abccba", 0, 5));
    }
    #[test]
    fn unit_length_palindrome() {
        assert!(is_palindrome("a", 0, 0));
    }
    #[test]
    fn zero_length_palindrome() {
        assert!(is_palindrome("", 0, 0));
    }

    #[test]
    fn rotation_true() {
        assert_eq!(is_rotation("abcd", "bcda"), true);
    }

    #[test]
    fn rotation_false() {
        assert_eq!(is_rotation("abcd", "bacd"), false);
    }

    #[test]
    fn single_duplicate() {
        assert_eq!(duplicate_finder("hello"), "l");
    }
    #[test]
    fn multiple_duplicates() {
        assert_eq!(duplicate_finder("hello world"), "lo");
    }
    #[test]
    fn unit_length_string_duplicate() {
        assert_eq!(duplicate_finder("a"), "");
    }
    #[test]
    fn empty_string_duplicate() {
        assert_eq!(duplicate_finder(""), "");
    }
}
