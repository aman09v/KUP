/// sort_array function is generic over T bound by PartialOrd trait
/// It sorts array of type T.
///
/// #Arguments
///
/// arr - an array which is to be sort.
///
/// #Return
///
/// Returns sorted array.

pub fn sort_array<T: std::cmp::PartialOrd>(arr: &mut [T]) -> &mut [T] {
    for index1 in 0..arr.len() {
        let mut small = index1;
        for index2 in (index1 + 1)..arr.len() {
            if arr[index2] < arr[small] {
                small = index2;
            }
        }
        arr.swap(small, index1);
    }
    arr
}
