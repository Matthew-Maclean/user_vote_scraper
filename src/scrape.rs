use hyper;

pub fn scrape_comments(client: &hyper::Client, token: &str, user: &str) -> Result<Vec<(String, Vote)>, ()>
{
    unimplemented!()
}

pub fn scrape_posts(client: &hyper::Client, token: &str, user: &str) -> Result<Vec<(String, Vote)>, ()>
{
    unimplemented!()
}

enum Vote
{
    Up,
    Down
}