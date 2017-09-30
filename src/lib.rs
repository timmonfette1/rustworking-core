extern crate oping;
extern crate reqwest;

use std::error;
use std::fmt;

use self::HttpError::{
    ErrorMessage,
};

// Error recieved from an HTTP call through reqwest
#[derive(Debug)]
pub enum HttpError {
    ErrorMessage(String),
}   

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorMessage(ref err) => write!(f, "{}", err),
        }
    }
}   

impl error::Error for HttpError {
    fn description(&self) -> &str {
        match *self {
            ErrorMessage(ref e) => &e,
        }
    }
}   

// Result recieved from an HTTP call through reqwest
pub type HttpResult<T> = Result<T, HttpError>;

pub mod rustytools;
pub mod utilities;
