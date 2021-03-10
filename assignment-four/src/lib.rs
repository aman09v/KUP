mod duplicate;
mod palindrome;
mod rotation;
mod test;

pub fn unused_warning_function() {
    duplicate::duplicate_finder("hello world");
    palindrome::is_palindrome("abba", 0, 3);
    rotation::is_rotation("abcd", "bcda");
}
