/// This function searches a pattern in a string.
///
/// #Arguments
///
/// str_to_check - a String type argument containing string to be checked.
/// pattern - a String type argument containing string to be matched.
///
/// #Return
///
/// Returns the String telling if the pattern exist and the index.

pub fn pattern_search(str_to_check: String, pattern: String) -> String {
    let vec_to_search: Vec<char> = str_to_check.chars().collect();
    let vec_pattern: Vec<char> = pattern.chars().collect();
    let mut count = 0;
    let mut match_index;
    let mut temp_index;
    for index in 0..(vec_to_search.len() - vec_pattern.len() + 1) {
        temp_index = index;
        match_index = index;
        for index_match in &vec_pattern {
            if index_match == &vec_to_search[temp_index] {
                count += 1;
            }

            if count == pattern.len() {
                return format!("pattern found at index {}", match_index);
            }
            temp_index += 1;
        }
        count = 0;
    }
    "no match".to_string()
}
