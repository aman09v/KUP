use log::info;

/// first_even function returns the first even number from vector.
///
/// #Arguments
///
/// seq : a vector to referenced i32 values.
///
/// #Return
///
/// Returns i32 value containing first even number.

pub fn first_even(seq: &[i32]) -> i32 {
    let mut index = 0;
    let value;
    loop {
        if seq[index] % 2 == 0 {
            value = seq[index];
            break;
        }
        index += 1;
    }
    info!("The first even is returned");
    value
}
