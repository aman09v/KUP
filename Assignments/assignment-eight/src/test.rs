#[cfg(test)]
mod tests {
    use crate::error_handler::even_error_handler::even_check;

    #[test]
    fn even_success() {
        env_logger::init();
        assert_eq!(even_check(2), "Even Number".to_string());
    }

    #[test]
    fn even_failure() {
        assert_eq!(even_check(3), "Error Handled".to_string());
    }
}
