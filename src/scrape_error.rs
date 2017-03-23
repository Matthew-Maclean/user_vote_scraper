use std::fmt;

#[derive(Copy, Clone)]
pub enum ScrapeError
{
    SendError,
    LimitHeaderError,
    ResetHeaderError,
    JsonError,
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
            &ScrapeError::JsonError => write!(f, "Error somewhere in the JSON response"),
            &ScrapeError::OtherError => write!(f, "Another type of error has occured")
        }
    }
}