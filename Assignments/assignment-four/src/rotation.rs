// This function checks if two string are rotation of itself
//
// #Arguments
//
// 'str1' - a str type reference to store one string to check
// 'str2' - a str type reference to store other string to check
//
// #Return
//
// Return a bool value denoting if the string are rotation of each other

pub fn is_rotation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }
    string1.repeat(2).contains(&string2)
}
