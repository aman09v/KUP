pub fn duplicate_finder(string_to_check: &str) -> String {
    //vector to store input string element wise
    let mut str_vec: Vec<char> = string_to_check.chars().collect();
    //string to store output
    let mut output: String = "".to_string();
    let mut count = 0;
    // outer loop to select one element of vector
    //to compare with all other elements using inner loop
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

#[cfg(test)]
mod tests {
    use crate::duplicate::duplicate_finder;

    #[test]
    fn single_duplicate() {
        assert_eq!(duplicate_finder("hello"), "l");
    }
    #[test]
    fn multiple_duplicates() {
        assert_eq!(duplicate_finder("hello world"), "lo");
    }
    #[test]
    fn unit_length() {
        assert_eq!(duplicate_finder("a"), "");
    }
    #[test]
    fn zero_length() {
        assert_eq!(duplicate_finder(""), "");
    }
}
