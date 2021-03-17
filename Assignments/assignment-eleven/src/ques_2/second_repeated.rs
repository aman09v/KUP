use crate::list::list_enum::List;
use crate::list::list_enum::List::Cons;
use crate::list::list_enum::List::Nil;
use log::info;

/// second_repeat_finder function finds the second element repeated.
///
/// #Arguments
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number containing second repeated number.

pub fn second_repeat_finder(list: List) -> i32 {
    recursion(-1, list, 0)
}

/// recursion function use recursion to match list object and find second repeated number.
///
/// #Arguments
///
/// 'prev' - An i32 variable containing the previous value in Cons tuple of List enum.
///
/// 'list'- A List enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number containing second repeated number.

pub fn recursion(prev: i32, list: List, counter: i32) -> i32 {
    info!("finds first repeated number");
    match list {
        Nil => -1,
        Cons(current, _) if current == prev && counter == 1 => current,
        Cons(current, list) if current == prev => recursion(current, *list, counter + 1),
        Cons(current, list) => recursion(current, *list, counter),
    }
}
