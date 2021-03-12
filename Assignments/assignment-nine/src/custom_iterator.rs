pub struct GeoProgression {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

impl Iterator for GeoProgression {
    type Item = i32;
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
}
