use serde_json;

use std::fmt;

pub enum ScrapeError
{
    SendError,
    LimitHeaderError,
    ResetHeaderError,
    JsonError(serde_json::Error),
    OtherError,
}

impl fmt::Display for ScrapeError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            &ScrapeError::SendError => write!(f, "Error while sending GET to reddit"),
            &ScrapeError::LimitHeaderError => write!(f, "Error while parsing limit header"),
            &ScrapeError::ResetHeaderError => write!(f, "Error while parsing reset header"),
            &ScrapeError::JsonError(ref e) => write!(f, "{}", e),
            &ScrapeError::OtherError => write!(f, "Another type of error has occured")
        }
    }
}

impl From<serde_json::Error> for ScrapeError
{
    fn from(err: serde_json::Error) -> ScrapeError
    {
        ScrapeError::JsonError(err)
    }
}