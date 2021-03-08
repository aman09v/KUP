/// This function finds the quadrant in which a point lies.
///
/// #Arguments
///
/// point - A tuple denoting x and y coordinates.
///
/// #Return
///
/// string representing quadrant.

pub fn position_finder(point: (i32, i32)) -> String {
    match point {
        (x, y) if x > 0 && y > 0 => {
            return format!(
                "Position::First(Coordinate::Abscissa({}), Coordinate::Ordinate({}))",
                point.0, point.1
            )
        }
        (x, y) if x < 0 && y > 0 => {
            return format!(
                "Position::Second(Coordinate::Abscissa({}), Coordinate::Ordinate({}))",
                point.0, point.1
            )
        }
        (x, y) if x < 0 && y < 0 => {
            return format!(
                "Position::Third(Coordinate::Abscissa({}), Coordinate::Ordinate({}))",
                point.0, point.1
            )
        }
        (x, y) if x > 0 && y < 0 => {
            return format!(
                "Position::Fourth(Coordinate::Abscissa({}), Coordinate::Ordinate({}))",
                point.0, point.1
            )
        }
        _ => {}
    }
    "no match".to_string()
}
