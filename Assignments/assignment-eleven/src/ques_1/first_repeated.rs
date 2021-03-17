use crate::list::list_enum::List;
use crate::list::list_enum::List::Cons;
use crate::list::list_enum::List::Nil;
use log::info;

/// first_repeat_finder function finds the first element repeated.
///
/// #Arguments
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number containing first repeated number.

pub fn first_repeat_finder(list: List) -> i32 {
    recursion(-1, list)
}

/// recursion function use recursion to match list object and find first repeated number.
///
/// #Arguments
///
/// 'prev' - An i32 variable containing the previous value in Cons tuple of List enum.
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number containing first repeated number.

pub fn recursion(prev: i32, list: List) -> i32 {
    info!("finds first repeated number");
    match list {
        Nil => -1,
        Cons(current, _) if current == prev => current,
        Cons(current, list) => recursion(current, *list),
    }
}
