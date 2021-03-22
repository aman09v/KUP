#[cfg(test)]
mod tests {
    use crate::ques_1::age_sum::sum_conditional;
    use crate::ques_2::add_duplicate::duplicate_element;
    use crate::ques_2::palindrome_check::is_palindrome;
    use crate::ques_2::remove_duplicate::delete_item;
    use crate::ques_2::remove_nth::drop_element;
    use crate::ques_2::return_even::first_even;
    use crate::ques_2::rev_list::reverse_list;
    use std::collections::HashMap;

    #[test]
    fn sum_age_check() {
        env_logger::init();
        let mut map = HashMap::new();
        map.insert("aman", 20);
        map.insert("karan", 19);
        map.insert("anurag", 30);
        map.insert("suresh", 25);
        map.insert("kailash", 18);
        assert_eq!(sum_conditional(map, "an").unwrap(), 69);
    }
    #[test]
    fn sum_age_check_different_value() {
        let mut map = HashMap::new();
        map.insert("aman", 20);
        map.insert("karan", 19);
        map.insert("anurag", 30);
        map.insert("suresh", 25);
        map.insert("kailash", 18);
        assert_eq!(sum_conditional(map, "sh").unwrap(), 43);
    }
    #[test]
    fn sum_age_check_no_value() {
        let mut map = HashMap::new();
        map.insert("aman", 20);
        map.insert("karan", 19);
        map.insert("anurag", 30);
        map.insert("suresh", 25);
        map.insert("kailash", 18);
        assert_eq!(sum_conditional(map, "").unwrap(), 112);
    }
    #[test]
    fn sum_age_check_fail() {
        let map = HashMap::new();
        assert_eq!(sum_conditional(map, ""), None);
    }
    #[test]
    fn add_duplicate_test() {
        let mut test_vec = vec![1, 2, 3];
        assert_eq!(
            duplicate_element(&mut test_vec).unwrap(),
            [1, 1, 2, 2, 3, 3]
        )
    }
    #[test]
    fn add_duplicate_fail() {
        let mut test_vec = vec![];
        assert_eq!(duplicate_element(&mut test_vec), None)
    }
    #[test]
    fn palindrome_success() {
        let test_vec = vec![1, 2, 2, 1];
        assert!(is_palindrome(test_vec, 0, 3).unwrap());
    }
    #[test]
    fn palindrome_fail() {
        let test_vec = vec![1, 2, 3, 4];
        assert!(!is_palindrome(test_vec, 0, 3).unwrap());
    }
    #[test]
    fn remove_duplicate_test() {
        let test_vec = vec![1, 2, 2, 1];
        assert_eq!(delete_item(test_vec).unwrap(), [1, 2, 1]);
    }
    #[test]
    fn remove_nth_test() {
        let test_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(
            drop_element(test_vec, 3).unwrap(),
            [1, 2, 4, 5, 6, 7, 8, 9, 10, 11]
        );
    }
    #[test]
    fn return_even_test() {
        let test_vec = vec![1, 21, 3, 4, 5];
        assert_eq!(first_even(&test_vec).unwrap(), 4);
    }
    #[test]
    fn reverse_list_test() {
        let test_vec = vec![1, 2, 3, 4, 5];
        assert_eq!(reverse_list(test_vec).unwrap(), [5, 4, 3, 2, 1]);
    }
}
