use log::info;

/// _drop_element function deletes every nth element from the vector.
///
/// #Arguments
///
/// vec : a vector containing i32 values.
/// num : a i32 containing value to be deleted.
///
/// #Return
///
/// Returns vector of i32 containing result.

pub fn _drop_element(mut vec: Vec<i32>, num: i32) -> Vec<i32> {
    let mut index = 0;
    while index < vec.len() {
        if num == vec[index] {
            vec.remove(index);
        }
        index += 1;
    }
    info!("The nth element is deleted");
    vec
}
