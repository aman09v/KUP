pub fn is_palindrome(str: &str, start: i32, end: i32) -> bool {
    //if empty string
    if str.len() == 0 {
        return true;
    }

    // If there is only one character
    if start == end {
        return true;
    }

    // If first and last
    // characters do not match
    if str.chars().nth(start as usize) != str.chars().nth(end as usize) {
        return false;
    }

    // If there are more than
    // two characters, check if
    // middle substring is also
    // palindrome or not.
    if start < end + 1 {
        return is_palindrome(str, start + 1, end - 1);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use crate::palindrome::is_palindrome;

    #[test]
    fn odd_length() {
        assert!(is_palindrome("amama", 0, 4));
    }
    #[test]
    fn even_length() {
        assert!(is_palindrome("abccba", 0, 5));
    }
    #[test]
    fn unit_length() {
        assert!(is_palindrome("a", 0, 0));
    }
    #[test]
    fn zero_length() {
        assert!(is_palindrome("", 0, 0));
    }
}
