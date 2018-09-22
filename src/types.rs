extern crate chrono;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;


use std::io;
use self::serde::ser::{ Serialize, Serializer };
use self::serde_json::error;
use self::uuid::Uuid;

use self::chrono::{ DateTime, Utc };

#[derive(Debug)]
pub enum Error {
    SerializationError(error::Error),
    IOError(io::Error),
}


pub trait Recordable
{
    fn timestamp(&self) -> DateTime<Utc>;
    fn tags(&self) -> Vec<String>;
    fn values(&self) -> Vec<String>;
}


#[derive(Clone, Debug, PartialEq)]
pub struct UniqueId(Uuid);

impl UniqueId {
    pub fn new() -> UniqueId {
        let id = Uuid::new_v4();
        UniqueId(id)
    }
}

impl Serialize for UniqueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.0.hyphenated().to_string())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Record<T: Clone + Recordable> {
    pub id: UniqueId,
    pub data: T,
}

impl <T> Record<T>
    where T: Clone + Recordable
{
    pub fn new(data: T) -> Record<T> {
        let id = UniqueId::new();
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

