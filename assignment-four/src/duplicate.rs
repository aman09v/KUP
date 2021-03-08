// This function finds duplicate character in string.
//
// #Arguments
//
// 'string_to_check' - a str type reference to store string to check
//
// #Return
//
// Return a string with duplicate characters

pub fn duplicate_finder(string_to_check: &str) -> String {
    let mut str_vec: Vec<char> = string_to_check.chars().collect();
    let mut output: String = "".to_string();
    let mut count = 0;
    for i in 0..string_to_check.len() {
        for j in i + 1..string_to_check.len() {
            if str_vec[i] == str_vec[j] && str_vec[i] != '0' {
                if count == 0 {
                    output.push(str_vec[i]);
                }
                count += 1;
                str_vec[j] = '0';
            }
        }
        count = 0;
    }
    output
}
