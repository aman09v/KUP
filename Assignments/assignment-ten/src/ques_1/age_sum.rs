use log::info;
use regex::Regex;
use std::collections::HashMap;
/// sum_conditional function calculates the sum of ages of matched persons.
///
/// #Arguments
///
/// map : a Hashmap have key as string type for name and values as i32 type for age.
///
/// str :  a string type containing pattern that need to be matched in name.
///
/// #Return
///
/// Returns i32 element containing sum of ages of matched persons.

pub fn sum_conditional(map: HashMap<&str, i32>, str: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(&*(r"".to_owned() + str)).unwrap();
    for person in map {
        if re.is_match(person.0) {
            sum += person.1
        }
    }
    info!("The sum of ages is returned");
    sum
}
