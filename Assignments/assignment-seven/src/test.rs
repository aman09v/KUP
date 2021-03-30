#[cfg(test)]
mod tests {
    use crate::ques1::pattern_searching::pattern_search;
    use crate::ques1::substring_generation::substring_generator;
    use crate::ques2::ques_two::desired_output;

    #[test]
    fn substring_check() {
        env_logger::init();
        assert_eq!(substring_generator("pa".to_string()), ["p", "pa", "a"]);
    }
    #[test]
    fn substring_check_() {
        assert_eq!(substring_generator("".to_string()), vec!["".to_string()]);
    }
    #[test]
    fn pattern_match_check() {
        assert_eq!(
            pattern_search("Pankaj Chaudhary".to_string(), "Cha".to_string()),
            "pattern found at index 7".to_string()
        );
    }
    #[test]
    fn pattern_do_no_exist() {
        assert_eq!(
            pattern_search("Pankaj Chaudhary".to_string(), "jhg".to_string()),
            "no match".to_string()
        );
    }
    #[test]
    fn desired_output_check() {
        assert_eq!(desired_output("jjdhid", "ikjhjk", "rtysgi"), "itdsgk");
    }
    #[test]
    fn desired_output_empty_check() {
        assert_eq!(desired_output("", "", ""), "");
    }
}
