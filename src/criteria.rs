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


pub fn exact_time(time: DateTime<Utc>) -> And<StartTime, EndTime> {
    And{
        lside: StartTime{ time, incl: true },
        rside: EndTime{ time, incl: true},
    }
}

