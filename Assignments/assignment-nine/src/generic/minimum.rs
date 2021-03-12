/// min function is generic over T bound by PartialOrd trait
/// and finds the minimum items passed as arguments.
///
/// #Arguments
///
/// item1 - a generic type argument containing data to be compared.
/// item2 - a generic type argument containing data to be compared.
///
/// #Return
///
/// Returns the minimum item

pub fn min<T: std::cmp::PartialOrd>(item1: T, item2: T) -> T {
    if item1 < item2 {
        item1
    } else {
        item2
    }
}
