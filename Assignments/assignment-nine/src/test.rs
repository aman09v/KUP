#[cfg(test)]
mod tests {
    use crate::custom_iterator::{GeoProgression, Iterator};
    use crate::generic::minimum::min;
    use crate::generic::sort::sort_array;

    #[test]
    fn min_check_integer() {
        assert_eq!(min(1, 2), 1);
    }
    #[test]
    fn min_check_float() {
        assert_eq!(min(1.2, 2.1), 1.2);
    }
    #[test]
    fn min_check_char() {
        assert_eq!(min('a', 'b'), 'a');
    }
    #[test]
    fn sort_integer() {
        let mut arr: [i32; 5] = [3, 2, 5, 1, 4];
        assert_eq!(sort_array(&mut arr), [1, 2, 3, 4, 5]);
    }
    #[test]
    fn sort_float() {
        let mut arr: [f32; 5] = [3.2, 2.2, 5.2, 1.2, 4.2];
        assert_eq!(sort_array(&mut arr), [1.2, 2.2, 3.2, 4.2, 5.2]);
    }
    #[test]
    fn sort_char() {
        let mut arr: [char; 5] = ['b', 'a', 'd', 'c', 'e'];
        assert_eq!(sort_array(&mut arr), ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_iter() {
        let mut gp = GeoProgression {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        let mut out: Vec<i32> = Vec::new();
        for i in gp.take(11) {
            out.push(i);
        }
        assert_eq!(out, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
    }
}
