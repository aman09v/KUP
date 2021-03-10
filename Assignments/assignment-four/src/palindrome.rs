// This function checks if a string is palindrome or not.
//
// #Arguments
//
// 'str' - a str type reference to store string to check
// 'start' - an i32 variable to store starting index
// 'end' -  an i32 variable to store end index
//
// #Return
//
// Return a bool value denoting if the string is palindrome or not

pub fn is_palindrome(str: &str, start: i32, end: i32) -> bool {
    if str.is_empty() {
        return true;
    }
    if start == end {
        return true;
    }
    if str.chars().nth(start as usize) != str.chars().nth(end as usize) {
        return false;
    }
    if start < end + 1 {
        return is_palindrome(str, start + 1, end - 1);
    }
    true
}
