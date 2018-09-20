extern crate chrono;
extern crate uuid;


use std::io;
use self::uuid::Uuid;

use self::chrono::{ DateTime, Utc };

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}


pub trait Recordable
{
    fn timestamp(&self) -> DateTime<Utc>;
    fn tags(&self) -> Vec<String>;
    fn values(&self) -> Vec<String>;
}


#[derive(Clone, Debug)]
pub struct Record<T: Clone + Recordable> {
    pub id: Uuid,
    pub data: T,
}

impl <T> Record<T>
    where T: Clone + Recordable
{
    pub fn new(data: T) -> Record<T> {
        let id = Uuid::new_v4();
        Record{id, data}
    }
}

impl <T> Recordable for Record<T>
    where T: Clone + Recordable
{
    fn timestamp(&self) -> DateTime<Utc> {
        self.data.timestamp()
    }
    fn tags(&self) -> Vec<String> {
        self.data.tags()
    }
    fn values(&self) -> Vec<String> {
        self.data.values()
    }
}

/*
impl <T> Recordable for & 'a Record<T>
    where T: Clone + Recordable
{
    fn timestamp(&self) -> DateTime<Utc> {
        self.data.timestamp()
    }
    fn tags(&self) -> DateTime<Utc> {
        self.data.tags()
    }
    fn values(&self) -> DateTime<Utc> {
        self.data.values()
    }
}
*/

