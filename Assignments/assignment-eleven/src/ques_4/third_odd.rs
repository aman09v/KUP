use crate::list::list_enum::List;
use crate::list::list_enum::List::Cons;
use crate::list::list_enum::List::Nil;
use log::info;

/// third_odd_finder function finds the third odd number in the list.
///
/// #Arguments
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number containing third odd number.

pub fn third_odd_finder(list: List) -> i32 {
    recursion(3, list)
}

/// recursion function use recursion to match list object and find first repeated number.
///
/// #Arguments
///
/// 'counter' - An i32 variable containing the count of current odd number in list.
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number containing third odd number.

pub fn recursion(counter: i32, list: List) -> i32 {
    info!("finds third odd number");
    match list {
        Nil => -1,
        Cons(current, _) if counter == 1 && current % 2 != 0 => current,
        Cons(current, list) if current % 2 != 0 => recursion(counter - 1, *list),
        Cons(_, list) => recursion(counter, *list),
    }
}
