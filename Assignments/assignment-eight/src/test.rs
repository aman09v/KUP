#[cfg(test)]
mod tests {
    use crate::even_error_handler::_even_check;

    #[test]
    fn even_success() {
        assert_eq!(_even_check(2), "Even Number".to_string());
    }

    #[test]
    fn even_failure() {
        assert_eq!(_even_check(3), "Error Handled".to_string());
    }
}
