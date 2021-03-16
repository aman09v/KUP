use log::info;

/// _drop_element function deletes nth element from the vector.
///
/// #Arguments
///
/// vec : a vector containing i32 values.
///
/// position :  an i32 value containing position of element to be deleted.
///
/// #Return
///
/// Returns vector of i32 containing result.

pub fn _drop_element(vec: &[i32], position: i32) -> Vec<i32> {
    let mut index: usize = 0;
    let mut answer: Vec<i32> = Vec::new();
    loop {
        if index == (position - 1) as usize {
            answer.extend_from_slice(&vec[..index]);
            break;
        }
        index += 1
    }
    answer.extend_from_slice(&vec[index + 1..]);
    info!("The nth element is deleted");
    answer
}
