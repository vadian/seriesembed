pub struct Interval<T> {
    start: T,
    start_incl: bool,
    end: T,
    end_incl: bool,
}

impl <T> Interval<T>
    where T: Clone + Ord
{
    pub fn new(start: T, start_incl: bool, end: T, end_incl: bool) -> Interval<T> {
        Interval{start, start_incl, end, end_incl}
    }

    pub fn exact(val: T) -> Interval<T> {
        Interval{start: val.clone(), start_incl: true, end: val, end_incl: true}
    }

    pub fn contains(&self, val: T) -> bool {
        match (self.start_incl, self.end_incl) {
            (true, true) => self.start <= val && val <= self.end,
            (true, false) => self.start <= val && val < self.end,
            (false, true) => self.start < val && val <= self.end,
            (false, false) => self.start < val && val < self.end,
        }
    }
}



