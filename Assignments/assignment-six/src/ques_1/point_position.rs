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
    XAxis(Coordinate, Coordinate),
    YAxis(Coordinate, Coordinate),
    Origin(Coordinate, Coordinate),
    None,
}

/// position_finder function finds the quadrant in which a point lies.
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
            Position::First(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        (x, y) if x < 0 && y > 0 => {
            Position::Second(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        (x, y) if x < 0 && y < 0 => {
            Position::Third(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        (x, y) if x > 0 && y < 0 => {
            Position::Fourth(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        (x, y) if x == 0 && y != 0 => {
            Position::YAxis(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        (x, y) if x != 0 && y == 0 => {
            Position::XAxis(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        (x, y) if x == 0 && y == 0 => {
            Position::Origin(Coordinate::Abscissa(point.0), Coordinate::Ordinate(point.1))
        }
        _ => Position::None,
    }
}
