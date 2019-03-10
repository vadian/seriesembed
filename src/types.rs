extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use std::fmt;
use std::io;
use self::serde::de;
use self::serde::de::{Deserialize, Deserializer, Visitor};
use self::serde::ser::{Serialize, Serializer};
use self::uuid::Uuid;

use self::chrono::{DateTime, Utc};

/// Errors for the database
#[derive(Debug)]
pub enum Error {
    /// Indicates that the UUID specified is invalid and cannot be parsed
    UUIDParseError(uuid::ParseError),

    /// Indicates an error in the JSON serialization
    SerializationError(serde_json::error::Error),

    /// Indicates an error in the JSON deserialization
    DeserializationError(serde_json::error::Error),

    /// Indicates a general IO error
    IOError(io::Error),
}


/// Any element to be put into the database needs to be Recordable. This is the common API that
/// will aid in searching and later in indexing records.
pub trait Recordable {
    /// The timestamp for the record.
    fn timestamp(&self) -> DateTime<Utc>;

    /// A list of string tags that can be used for indexing. This list defined per-type.
    fn tags(&self) -> Vec<String>;

    /// [Deprecated] This was never put into use and is probably not at all useful.
    fn values(&self) -> Vec<String>;
}


/// Uniquely identifies a record.
///
/// This is a wrapper around a basic uuid with some extra convenience methods.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UniqueId(Uuid);

impl UniqueId {
    /// Create a new V4 UUID (this is the most common type in use these days).
    pub fn new() -> UniqueId {
        let id = Uuid::new_v4();
        UniqueId(id)
    }

    /// Parse a UUID from a string. Raise UUIDParseError if the parsing fails.
    pub fn from_str(val: &str) -> Result<UniqueId, Error> {
        Uuid::parse_str(val).map(UniqueId).map_err(|err| {
            Error::UUIDParseError(err)
        })
    }
}

struct UniqueIdVisitor;

impl<'de> Visitor<'de> for UniqueIdVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing a uuid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(String::from(v))
    }
}

impl<'de> Deserialize<'de> for UniqueId {
    fn deserialize<D>(deserializer: D) -> Result<UniqueId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = deserializer.deserialize_str(UniqueIdVisitor)?;
        Uuid::parse_str(&val).map(UniqueId).map_err(|err| {
            de::Error::custom(format!(
                "unexpected error found with input: {}",
                err.to_string()
            ))
        })
    }
}

impl Serialize for UniqueId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.hyphenated().to_string())
    }
}

/// Every record contains a unique ID and then the primary data, which itself must implementd the
/// Recordable trait.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record<T: Clone + Recordable> {
    pub id: UniqueId,
    pub data: T,
}

impl<T> Record<T>
where
    T: Clone + Recordable,
{
    pub fn new(data: T) -> Record<T> {
        let id = UniqueId::new();
        Record { id, data }
    }
}

impl<T> Recordable for Record<T>
where
    T: Clone + Recordable,
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
