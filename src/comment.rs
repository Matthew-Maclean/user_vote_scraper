use hyper;
use serde_json;

use std::thread;

use scrape_error::ScrapeError;

#[derive(Deserialize)]
struct CommentResponse
{
    data: CommentListing,
}

#[derive(Deserialize)]
struct CommentListing
{
    children: Vec<CommentWrapper>,
    after: Option<String>,
}

#[derive(Deserialize)]
struct CommentWrapper
{
    data: Comment
}

#[derive(Deserialize)]
pub struct Comment
{
    pub id: String,
    pub link_id: String,
    pub likes: Option<bool>,
}

impl Comment
{
    pub fn scrape(client: &hyper::Client, token: &str, user: &str, limit: i32) -> Result<Vec<Comment>, ScrapeError>
    {
        let headers =
        {
            let mut h = hyper::header::Headers::new();
            h.set(
                hyper::header::Authorization(
                    hyper::header::Bearer
                    {
                        token: token.to_owned()
                    }
                )
            );
            h.set(
                hyper::header::UserAgent("windows: user_vote_scraper (pre-alpha)".to_owned())
            );
            h
        };

        let url_base = format!("https://oauth.reddit.com/user/{user}/comments/.json?limit=100", user = user);

        let mut count: i32 = 0;
        let mut after: Option<String> = None;
        let mut threads = Vec::new();

        while count < limit
        {
            let url = if let Some(ref id) = after
            {
                format!("{base}&after={after}", base = url_base, after = id)
            }
            else
            {
                url_base.clone()
            };

            let res = match client.get(&url).headers(headers.clone()).send()
            {
                Ok(r) => r,
                Err(_) => return Err(ScrapeError::SendError)
            };

            let remaining = match res.headers.get_raw("X-Ratelimit-Remaining")
            {
                Some(bytes_slice) =>
                {
                    if bytes_slice.len() == 0
                    {
                        return Err(ScrapeError::LimitHeaderError)
                    }
                    let bytes = bytes_slice[0].clone();

                    let string = match String::from_utf8(bytes)
                    {
                        Ok(s) => s,
                        Err(_) => return Err(ScrapeError::LimitHeaderError)
                    };

                    let num = match string.parse::<f64>() // why is the number of remaining requests a real number? I have no idea
                    {
                        Ok(n) => n,
                        Err(_) => return Err(ScrapeError::LimitHeaderError)
                    };

                    num
                },
                None => return Err(ScrapeError::LimitHeaderError)
            };
            
            let reset = match res.headers.get_raw("X-Ratelimit-Reset")
            {
                Some(bytes_slice) =>
                {
                    if bytes_slice.len() == 0
                    {
                        return Err(ScrapeError::LimitHeaderError)
                    }
                    let bytes = bytes_slice[0].clone();

                    let string = match String::from_utf8(bytes)
                    {
                        Ok(s) => s,
                        Err(_) => return Err(ScrapeError::ResetHeaderError)
                    };

                    let num = match string.parse::<f64>()
                    {
                        Ok(n) => n,
                        Err(_) => return Err(ScrapeError::ResetHeaderError)
                    };

                    num
                },
                None => return Err(ScrapeError::ResetHeaderError)
            };

            let response = serde_json::from_reader::<_, CommentResponse>(res)?;

            let new_after = response.data.after.clone();

            let handle = thread::spawn(move ||
            {
                let mut voted = Vec::new();

                for comment in response.data.children.into_iter()
                {
                    if comment.data.likes.is_some()
                    {
                        voted.push(comment.data);
                    }
                }
                voted
            });

            threads.push(handle);

            match new_after
            {
                Some(new_id) => match after
                {
                    Some(ref old_id) if &new_id == old_id => break,
                    _ => after = Some(new_id)
                },
                None => break
            }

            if remaining < 2.0
            {
                thread::sleep(::std::time::Duration::from_secs(reset as u64));
            }

            count += 100;
        }

        let mut comments = Vec::new();

        for thread in threads.into_iter()
        {
            match thread.join()
            {
                Ok(mut v) => comments.append(&mut v),
                Err(_) => return Err(ScrapeError::OtherError)
            }
        }

        Ok(comments)
    }
}