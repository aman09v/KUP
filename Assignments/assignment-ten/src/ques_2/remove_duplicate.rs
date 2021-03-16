use log::info;

/// _delete_item function delete concurrent duplicate elements.
///
/// #Arguments
///
/// vec : a vector containing i32 values.
///
/// #Return
///
/// Returns vector of i32 values with deleted duplicate elements.

pub fn _delete_item(vec: Vec<i32>) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for item in vec {
        let top = out.last();
        match top {
            Some(value) => {
                if item != *value {
                    out.push(item);
                }
            }
            None => out.push(item),
        }
    }
    info!("This is removing duplicates");
    out
}
