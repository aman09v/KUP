use std::clone::Clone;
use std::fmt::Debug;
use std::marker::Copy;
#[derive(Copy, Clone, Debug)]
pub struct GeoProgression {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

impl Iterator for GeoProgression {
    type Item = Vec<i32>;
    /// next function generates an iterator for n elements in GP.
    ///
    /// #Arguments
    ///
    /// size - an i32 argument denoting number of elements in iterator.
    ///
    /// #Return
    ///
    /// Returns the Vector containing 'size' numbers in GP.
    fn next(&mut self) -> Option<Vec<i32>> {
        let mut out: Vec<i32> = Vec::new();
        for _index in 0..11 {
            self.current_number = self.first_number * self.ratio.pow(_index);
            out.push(self.current_number);
        }
        Some(out)
    }
}
