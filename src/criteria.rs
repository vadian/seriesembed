extern crate chrono;

use self::chrono::{ DateTime, Utc };
use types::Recordable;

pub trait Criteria {
    fn apply<T: Recordable>(&self, record: &T) -> bool;
}


pub struct And<A: Criteria, B: Criteria> {
    pub lside: A,
    pub rside: B,
}


impl <A, B> Criteria for And<A, B>
    where A: Criteria,
          B: Criteria
{
    fn apply<T: Recordable>(&self, record: &T) -> bool {
        self.lside.apply(record) && self.rside.apply(record)
    }
}


pub struct Or<A: Criteria, B: Criteria> {
    pub lside: A,
    pub rside: B,
}


pub struct StartTime {
    pub time: DateTime<Utc>,
    pub incl: bool,
}


impl Criteria for StartTime {
    fn apply<T: Recordable>(&self, record: &T) -> bool {
        if self.incl {
            record.timestamp() >= self.time
        } else {
            record.timestamp() > self.time
        }
    }
}


pub struct EndTime {
    pub time: DateTime<Utc>,
    pub incl: bool,
}


impl Criteria for EndTime {
    fn apply<T: Recordable>(&self, record: &T) -> bool {
        if self.incl {
            record.timestamp() <= self.time
        } else {
            record.timestamp() < self.time
        }
    }
}


pub struct Tags {
    pub tags: Vec<String>,
}

impl Criteria for Tags {
    fn apply<T: Recordable>(&self, record: &T) -> bool {
        let record_tags = record.tags();
        let mismatched_tags: Vec<bool> = self.tags.iter().map(|v| record_tags.contains(v)).filter(|v| !v).collect();
        mismatched_tags.len() == 0
    }
}


pub fn exact_time(time: DateTime<Utc>) -> And<StartTime, EndTime> {
    And{
        lside: StartTime{ time, incl: true },
        rside: EndTime{ time, incl: true},
    }
}


pub fn time_range(start: DateTime<Utc>,
                  start_incl: bool,
                  end: DateTime<Utc>,
                  end_incl: bool) -> And<StartTime, EndTime> {
    And{
        lside: StartTime{ time: start, incl: start_incl },
        rside: EndTime{ time: end, incl: end_incl },
    }
}

