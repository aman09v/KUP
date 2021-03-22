pub trait Iterator {
    fn next(&mut self) -> Option<i32>;
    fn take(&mut self, size: i32) -> Option<Vec<i32>>;
}

pub struct GeoProgression {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

impl Iterator for GeoProgression {
    /// next function calculates next value in GP.
    ///
    /// #Arguments
    ///
    /// No arguments.
    ///
    /// #Return
    ///
    /// Returns the Option enum object containing next value of GP.
    fn next(&mut self) -> Option<i32> {
        let out = self.current_number;
        self.current_number *= self.ratio;
        Some(out)
    }
    /// take function generates an iterator for n elements in GP.
    ///
    /// #Arguments
    ///
    /// size - an i32 argument denoting number of elements in iterator.
    ///
    /// #Return
    ///
    /// Returns the Vector containing 'size' numbers in GP.
    fn take(&mut self, size: i32) -> Option<Vec<i32>> {
        let mut out: Vec<i32> = Vec::new();
        for _index in 0..size {
            out.push(self.next().unwrap());
        }
        Some(out)
    }
}
