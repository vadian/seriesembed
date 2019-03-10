// NOTE: this module is a candidate for extraction into its own crate, or should be replaced with
// an existing crate.

/// Specify an interval across the data type T
pub struct Interval<T> {
    start: T,
    start_incl: bool,
    end: T,
    end_incl: bool,
}

impl<T> Interval<T>
where
    T: Clone + Ord,
{
    /// Create a new interval from the start value to the end value, specifying inclusivity on
    /// either end of the interval.
    pub fn new(start: T, start_incl: bool, end: T, end_incl: bool) -> Interval<T> {
        Interval {
            start,
            start_incl,
            end,
            end_incl,
        }
    }

    /// Create an interval that matches *exactly* the specified value.
    pub fn exact(val: T) -> Interval<T> {
        Interval {
            start: val.clone(),
            start_incl: true,
            end: val,
            end_incl: true,
        }
    }

    /// Test whether a value is included in the specified interval.
    pub fn contains(&self, val: T) -> bool {
        match (self.start_incl, self.end_incl) {
            (true, true) => self.start <= val && val <= self.end,
            (true, false) => self.start <= val && val < self.end,
            (false, true) => self.start < val && val <= self.end,
            (false, false) => self.start < val && val < self.end,
        }
    }
}
