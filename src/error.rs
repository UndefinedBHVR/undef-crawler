use serde::Serialize;
use std::{error::Error, fmt::Display};
//Currently, the Crawler cannot error. Making this redundant. 
#[derive(Serialize, Debug)]
pub enum CrawlerError {
    Unknown,
}
impl Display for CrawlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "An unknown server error has occured!"),
        }
    }
}
impl Error for CrawlerError {}
