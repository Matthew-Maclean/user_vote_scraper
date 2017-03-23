use std::fmt;

#[derive(Copy, Clone)]
pub enum Vote
{
    Up,
    Down
}

impl fmt::Display for Vote
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            &Vote::Up => write!(f, "up vote"),
            &Vote::Down => write!(f, "down vote")
        }
    }
}