use crate::list::list_enum::List;
use crate::list::list_enum::List::Cons;
use crate::list::list_enum::List::Nil;
use log::info;

/// nth_finder function finds the nth element in list.
///
/// #Arguments
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// 'position' - An i32 variable containing the position of element to fetch.
///
/// #Return
///
/// Return the i32 number containing nth number of list.

pub fn nth_finder(position: i32, list: List) -> i32 {
    recursion(position - 1, list, 0)
}

/// recursion function use recursion to match list object and find nth number.
///
/// #Arguments
///
/// 'position' - An i32 variable containing the position of number to find in list.
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// 'counter' - An i32 variable containing the position of current number in list.
///
/// #Return
///
/// Return the i32 number containing nth number.

pub fn recursion(position: i32, list: List, counter: i32) -> i32 {
    info!("finds number at {} postion", position);
    match list {
        Nil => -1,
        Cons(current, _) if counter == position => current,
        Cons(_, list) => recursion(position, *list, counter + 1),
    }
}
