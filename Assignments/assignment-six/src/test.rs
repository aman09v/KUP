#[cfg(test)]
mod tests {
    use crate::ip_enum::{classify, IpClass};
    use crate::point_position::{position_finder, Coordinate, Position};
    #[test]
    fn quadrant_check() {
        let input = vec![(1, 2), (-1, 2), (1, -2), (-1, -2)];
        assert_eq!(
            position_finder(input[0]),
            Position::First(Coordinate::Abscissa(1), Coordinate::Ordinate(2))
        );
        assert_eq!(
            position_finder(input[1]),
            Position::Second(Coordinate::Abscissa(-1), Coordinate::Ordinate(2))
        );
        assert_eq!(
            position_finder(input[2]),
            Position::Fourth(Coordinate::Abscissa(1), Coordinate::Ordinate(-2))
        );
        assert_eq!(
            position_finder(input[3]),
            Position::Third(Coordinate::Abscissa(-1), Coordinate::Ordinate(-2))
        );
        assert_eq!(position_finder((0, 0)), Position::Origin);
    }
    #[test]
    fn ip_class_check() {
        let input = vec![
            (192, 0, 1, 1),
            (230, 45, 6, 7),
            (170, 45, 23, 45),
            (198, 5, 6, 4),
            (102, 1, 3, 4),
        ];
        assert_eq!(classify(input[0]), IpClass::ClassC("192.0.1.1".to_string()));
        assert_eq!(
            classify(input[1]),
            IpClass::ClassD("230.45.6.7".to_string())
        );
        assert_eq!(
            classify(input[2]),
            IpClass::ClassB("170.45.23.45".to_string())
        );
        assert_eq!(classify(input[3]), IpClass::ClassC("198.5.6.4".to_string()));
        assert_eq!(classify(input[4]), IpClass::ClassA("102.1.3.4".to_string()));
    }
    #[test]
    fn no_class_ip() {
        let input = (500, 1, 2, 3);
        assert_eq!(classify(input), IpClass::None);
    }
}
