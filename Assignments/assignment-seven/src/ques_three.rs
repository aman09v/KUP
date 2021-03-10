/// This function gets desired output for even and odd position of strings.
///
/// #Arguments
///
/// input_str1 - a String type argument for first string.
/// input_str2 - a String type argument for second string.
/// input_str3 - a String type argument for third string.
///
/// #Return
///
/// Returns the String with desired output.

pub fn desired_output(input_str1: &str, input_str2: &str, input_str3: &str) -> String {
    let mut count = 0;
    let mut index: usize = 0;
    let mut output: Vec<char> = Vec::new();
    while index < input_str1.len() {
        if count % 2 == 0 {
            let temp_result = if input_str1.chars().nth(index) < input_str2.chars().nth(index) {
                input_str1.chars().nth(index)
            } else {
                input_str2.chars().nth(index)
            };
            let result = if temp_result < input_str3.chars().nth(index) {
                temp_result
            } else {
                input_str3.chars().nth(index)
            };
            if let Some(_t) = result {
                output.push(result.unwrap())
            }
        } else {
            let temp_result = if input_str1.chars().nth(index) < input_str2.chars().nth(index) {
                input_str2.chars().nth(index)
            } else {
                input_str1.chars().nth(index)
            };
            let result = if temp_result > input_str3.chars().nth(index) {
                temp_result
            } else {
                input_str3.chars().nth(index)
            };
            if let Some(_t) = result {
                output.push(result.unwrap())
            }
        }

        index += 1;
        count += 1;
    }
    output.iter().collect()
}
