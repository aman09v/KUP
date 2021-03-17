use log::info;

/// reverse_list function reverse the vector.
///
/// #Arguments
///
/// vec_to_reverse : a vector containing i32 values to be reversed.
///
/// #Return
///
/// Returns vector containing reversed i32 values.

pub fn reverse_list(mut vec_to_reverse: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let mut len = vec_to_reverse.len();
    while len > 0 {
        let last = vec_to_reverse.last();
        match last {
            Some(element) => {
                out.push(*element);
                vec_to_reverse.pop();
            }
            None => log::info!("empty"),
        }
        len -= 1;
    }
    info!("The list has been reversed");
    out
}
