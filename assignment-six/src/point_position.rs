#[derive(PartialEq, Eq, Debug)]
pub enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}
#[derive(PartialEq, Eq, Debug)]
pub enum Position {
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
    Origin,
}

/// This function finds the quadrant in which a point lies.
///
/// #Arguments
///
/// point - A tuple denoting x and y coordinates.
///
/// #Return
///
/// Position enum representing quadrant.

pub fn position_finder(point: (i32, i32)) -> Position {
    match point {
        (x, y) if x > 0 && y > 0 => {
            return Position::First(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1));
        }
        (x, y) if x < 0 && y > 0 => {
            return Position::Second(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1));
        }
        (x, y) if x < 0 && y < 0 => {
            return Position::Third(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1));
        }
        (x, y) if x > 0 && y < 0 => {
            return Position::Fourth(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1));
        }
        _ => {}
    }
    Position::Origin
}
