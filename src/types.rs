extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use std::io;
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
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
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
