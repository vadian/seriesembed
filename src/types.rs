extern crate chrono;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use std::fmt;
use std::io;
use self::serde::de;
use self::serde::de::{ Deserialize, Deserializer, Visitor };
use self::serde::ser::{ Serialize, Serializer };
use self::uuid::Uuid;

use self::chrono::{ DateTime, Utc };

#[derive(Debug)]
pub enum Error {
    SerializationError(serde_json::error::Error),
    DeserializationError(serde_json::error::Error),
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

struct UniqueIdVisitor;

impl <'de> Visitor<'de> for UniqueIdVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing a uuid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(String::from(v))
    }
}

impl <'de> Deserialize<'de> for UniqueId {
    fn deserialize<D>(deserializer: D) -> Result<UniqueId, D::Error>
        where D: Deserializer<'de>
    {
        let val = deserializer.deserialize_str(UniqueIdVisitor)?;
        Uuid::parse_str(&val)
            .map(UniqueId)
            .map_err(|err| de::Error::custom(format!("unexpected error found with input: {}", err.to_string())))
    }
}

impl Serialize for UniqueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.0.hyphenated().to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

