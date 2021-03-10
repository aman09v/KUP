/// This function generates all possible substrings of a string.
///
/// #Arguments
///
/// input_str - String type object whose substrings are generated.
///
/// #Return
///
/// Returns a vector containing substrings.

pub fn substring_generator(input_str: String) -> Vec<String> {
    if input_str.is_empty() {
        return vec!["".to_string()];
    }
    let mut output: Vec<String> = Vec::new();
    let mut substr: &str;
    for i in 0..input_str.len() {
        for j in i..input_str.len() {
            substr = &input_str[i..(j + 1)];
            output.push(substr.to_string());
        }
    }
    output
}
