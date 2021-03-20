#[cfg(test)]
mod tests {
    use crate::ques_1::point_position::{position_finder, Coordinate, Position};
    use crate::ques_2::ip_enum::{classify, IpClass};
    #[test]
    fn first_quadrant_check() {
        env_logger::init();
        assert_eq!(
            position_finder((1, 2)),
            Position::First(Coordinate::Abscissa(1), Coordinate::Ordinate(2))
        );
    }
    #[test]
    fn second_quadrant_check() {
        assert_eq!(
            position_finder((-1, 2)),
            Position::Second(Coordinate::Abscissa(-1), Coordinate::Ordinate(2))
        );
    }
    #[test]
    fn fourth_quadrant_check() {
        assert_eq!(
            position_finder((1, -2)),
            Position::Fourth(Coordinate::Abscissa(1), Coordinate::Ordinate(-2))
        );
    }
    #[test]
    fn third_quadrant_check() {
        assert_eq!(
            position_finder((-1, -2)),
            Position::Third(Coordinate::Abscissa(-1), Coordinate::Ordinate(-2))
        );
    }
    #[test]
    fn origin_check() {
        assert_eq!(
            position_finder((0, 0)),
            Position::Origin(Coordinate::Abscissa(0), Coordinate::Ordinate(0))
        );
    }
    #[test]
    fn x_axis_check() {
        assert_eq!(
            position_finder((3, 0)),
            Position::XAxis(Coordinate::Abscissa(3), Coordinate::Ordinate(0))
        );
    }
    #[test]
    fn y_axis_check() {
        assert_eq!(
            position_finder((0, 3)),
            Position::YAxis(Coordinate::Abscissa(0), Coordinate::Ordinate(3))
        );
    }
    #[test]
    fn class_c_check() {
        assert_eq!(
            classify((192, 0, 1, 1)),
            IpClass::ClassC("192.0.1.1".to_string())
        );
    }
    #[test]
    fn class_d_check() {
        assert_eq!(
            classify((230, 45, 6, 7)),
            IpClass::ClassD("230.45.6.7".to_string())
        );
    }
    #[test]
    fn class_b_check() {
        assert_eq!(
            classify((170, 45, 23, 45)),
            IpClass::ClassB("170.45.23.45".to_string())
        );
    }
    #[test]
    fn class_a_check() {
        assert_eq!(
            classify((102, 1, 3, 4)),
            IpClass::ClassA("102.1.3.4".to_string())
        );
    }
    #[test]
    fn class_check_failure() {
        assert_eq!(classify((102, 1000, 3000, 4000)), IpClass::None);
    }
    #[test]
    fn no_class_ip_check() {
        let input = (255, 1, 2, 255);
        assert_eq!(classify(input), IpClass::None);
    }
}
