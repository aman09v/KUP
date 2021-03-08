mod ip_enum;
mod point_position;
mod test;

/// This function uses functions to remove warning
///
/// #Arguments
///
/// No arguments
///
/// #Return
///
/// No return type

pub fn remove_unused_warnings() {
    point_position::position_finder((1, 2));
    ip_enum::classify((192, 1, 2, 3));
}
