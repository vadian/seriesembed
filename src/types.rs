extern crate chrono;
extern crate uuid;


use std::io;
use self::uuid::Uuid;

use self::chrono::{ DateTime, Utc };

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}


pub trait Record
{
    fn id(&self) -> Option<Uuid>;
    fn set_id(&mut self, Uuid);
    fn timestamp(&self) -> DateTime<Utc>;
    fn tags(&self) -> Vec<String>;
    fn values(&self) -> Vec<String>;
}



