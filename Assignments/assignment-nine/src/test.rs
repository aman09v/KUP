#[cfg(test)]
mod tests {
    use crate::custom_iterator::GeoProgression;
    use crate::generic::minimum::min;
    use crate::generic::sort::sort_array;

    #[test]
    fn min_check_integer() {
        env_logger::init();
        assert_eq!(min(1, 2).unwrap(), 1);
    }
    #[test]
    fn min_check_float() {
        assert_eq!(min(1.2, 2.1).unwrap(), 1.2);
    }
    #[test]
    fn min_check_char() {
        assert_eq!(min('a', 'b').unwrap(), 'a');
    }
    #[test]
    fn min_check_fail() {
        assert_eq!(min('a', 'a'), Err("equal value".to_string()));
    }
    #[test]
    fn sort_integer() {
        let mut arr: [i32; 5] = [3, 2, 5, 1, 4];
        assert_eq!(sort_array(&mut arr).unwrap(), [1, 2, 3, 4, 5]);
    }
    #[test]
    fn sort_float() {
        let mut arr: [f32; 5] = [3.2, 2.2, 5.2, 1.2, 4.2];
        assert_eq!(sort_array(&mut arr).unwrap(), [1.2, 2.2, 3.2, 4.2, 5.2]);
    }
    #[test]
    fn sort_char() {
        let mut arr: [char; 5] = ['b', 'a', 'd', 'c', 'e'];
        assert_eq!(sort_array(&mut arr).unwrap(), ['a', 'b', 'c', 'd', 'e']);
    }
    #[test]
    fn test_iter() {
        let mut gp = GeoProgression {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            gp.next(),
            Some(vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024])
        );
    }
}
