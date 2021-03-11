mod ques_three;
mod test;

pub mod ques1 {
    pub mod pattern_searching;
    pub mod substring_generation;
}

/// remove_unused_warnings function uses functions to remove warning
///
/// #Arguments
///
/// No arguments
///
/// #Return
///
/// No return type

pub fn remove_unused_warning() {
    ques1::substring_generation::substring_generator("pa".to_string());
    ques1::pattern_searching::pattern_search("Pankaj Chaudhary".to_string(), "Cha".to_string());
    ques_three::desired_output("", "", "");
}
