use log::error;

/// even_check function handles the output generated by _return_result.
///
/// #Arguments
///
/// num - a i32 type argument containing number to be checked.
///
/// #Return
///
/// Returns the String handling error.

pub fn even_check(num: i32) -> String {
    let result = return_result(num);
    match result {
        Ok(_even) => _even,
        Err(_err) => {
            error!("Error occurred and handled");
            "Error Handled".to_string()
        }
    }
}

/// return_result function checks for even or odd value.
///
/// #Arguments
///
/// num - a i32 type argument containing number to be checked.
///
/// #Return
///
/// Returns the appropriate Result enum object.

pub fn return_result(num: i32) -> Result<String, String> {
    if num % 2 == 0 {
        Result::Ok("Even Number".to_string())
    } else {
        Result::Err("Odd Number".to_string())
    }
}