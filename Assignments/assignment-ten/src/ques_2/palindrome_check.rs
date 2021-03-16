use log::info;

/// _is_palindrome function checks if a vector is palindrome or not.
///
/// #Arguments
///
/// vec_to_check : a vector containing i32 values to be checked.
///
/// start :  a usize variable containing starting index.
///
/// end :  a usize variable containing ending index.
///
/// #Return
///
/// Returns bool value denoting if vector is palindrome or not.

pub fn _is_palindrome(vec_to_check: Vec<i32>, start: usize, end: usize) -> bool {
    if start == end {
        return true;
    }

    if vec_to_check[start] != vec_to_check[end] {
        return false;
    }

    if start < end + 1 {
        return _is_palindrome(vec_to_check, start + 1, end - 1);
    }
    info!("The palindrome is checked");

    true
}