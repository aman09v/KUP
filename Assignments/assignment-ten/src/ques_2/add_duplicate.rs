use log::info;

/// duplicate_element function adds duplicate of each element.
///
/// #Arguments
///
/// vec : a vector containing i32 values.
///
/// #Return
///
/// Returns vector containing duplicate values.

pub fn duplicate_element(vec: &mut Vec<i32>) -> Option<Vec<i32>> {
    let size = vec.len();
    if vec.is_empty() {
        return None;
    }
    for item in 0..size {
        let value = vec[item];
        println!("{}", value);
        vec.push(value);
        vec.push(value);
    }
    info!("The duplicate element is inserted");
    Some(vec[size..].to_vec())
}
