#[cfg(test)]
mod tests {
    use crate::ques_1::age_sum::_sum_conditional;
    use crate::ques_2::add_duplicate::_duplicate_element;
    use crate::ques_2::palindrome_check::_is_palindrome;
    use crate::ques_2::remove_duplicate::_delete_item;
    use crate::ques_2::remove_nth::_drop_element;
    use crate::ques_2::return_even::_first_even;
    use crate::ques_2::rev_list::_reverse_list;
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
        assert_eq!(_sum_conditional(map, "an"), 69);
    }
    #[test]
    fn sum_age_check_different_value() {
        let mut map = HashMap::new();
        map.insert("aman", 20);
        map.insert("karan", 19);
        map.insert("anurag", 30);
        map.insert("suresh", 25);
        map.insert("kailash", 18);
        assert_eq!(_sum_conditional(map, "sh"), 43);
    }
    #[test]
    fn sum_age_check_no_value() {
        let mut map = HashMap::new();
        map.insert("aman", 20);
        map.insert("karan", 19);
        map.insert("anurag", 30);
        map.insert("suresh", 25);
        map.insert("kailash", 18);
        assert_eq!(_sum_conditional(map, ""), 112);
    }
    #[test]
    fn add_duplicate_test() {
        let mut test_vec = vec![1, 2, 3];
        assert_eq!(_duplicate_element(&mut test_vec), [1, 1, 2, 2, 3, 3])
    }
    #[test]
    fn palindrome_test() {
        let test_vec = vec![1, 2, 2, 1];
        assert!(_is_palindrome(test_vec, 0, 3));
    }
    #[test]
    fn remove_duplicate_test() {
        let test_vec = vec![1, 2, 2, 1];
        assert_eq!(_delete_item(test_vec), [1, 2, 1]);
    }
    #[test]
    fn remove_nth_test() {
        let test_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(_drop_element(test_vec, 3), [1, 2, 4, 5, 6, 7, 8, 9, 10, 11]);
    }
    #[test]
    fn return_even_test() {
        let test_vec = vec![1, 21, 3, 4, 5];
        assert_eq!(_first_even(&test_vec), 4);
    }
    #[test]
    fn reverse_list_test() {
        let test_vec = vec![1, 2, 3, 4, 5];
        assert_eq!(_reverse_list(test_vec), [5, 4, 3, 2, 1]);
    }
}
