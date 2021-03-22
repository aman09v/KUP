#[cfg(test)]
mod tests {
    use crate::list::list_enum::List::Cons;
    use crate::list::list_enum::List::Nil;
    use crate::ques_1::first_repeated::first_repeat_finder;
    use crate::ques_2::second_repeated::second_repeat_finder;
    use crate::ques_3::element_search::nth_finder;
    use crate::ques_4::third_odd::third_odd_finder;

    #[test]
    fn first_repeat_finder_success() {
        env_logger::init();
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(4, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(first_repeat_finder(test_list), 2);
    }
    #[test]
    fn first_repeat_finder_failure() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(first_repeat_finder(test_list), -1);
    }
    #[test]
    fn second_repeat_finder_success() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(4, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(second_repeat_finder(test_list), 4);
    }
    #[test]
    fn second_repeat_finder_failure() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(second_repeat_finder(test_list), -1);
    }
    #[test]
    fn element_search_check() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(4, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(nth_finder(4, test_list), 3);
    }
    #[test]
    fn third_odd_check_success() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        assert_eq!(third_odd_finder(test_list), 3);
    }
    #[test]
    fn third_odd_check_failure() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(4, Box::new(Cons(4, Box::new(Cons(6, Box::new(Nil))))))),
            )),
        );
        assert_eq!(third_odd_finder(test_list), -1);
    }
}
