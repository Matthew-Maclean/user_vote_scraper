use hyper;
use rustc_serialize;

use std::thread;
use std::io::Read;

#[allow(dead_code)]
pub fn scrape_comments(client: &hyper::Client, token: &str, user: &str, limit: i32) -> Result<Vec<(String, Vote)>, ScrapeError>
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
        h
    };

    let url_base = format!("https://oauth.reddit.com/user/{user}/comments/.json/?limit=100", user = user);

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

        let mut res = match client.get(&url).headers(headers.clone()).send()
        {
            Ok(r) => r,
            Err(_) => return Err(ScrapeError::SendError)
        };

        let json = match rustc_serialize::json::Json::from_reader(&mut res)
        {
            Ok(j) => j,
            Err(_) => return Err(ScrapeError::JsonError)
        };

        let after_tmp = match json.find("data").and_then(|x| x.find("after")).and_then(|x| x.as_string())
        {
            Some(s) => s.to_owned(),
            None => return Err(ScrapeError::JsonError)
        };

        if let Some(id) = after
        {
            if id == after_tmp
            {
                break;
            }
        }

        after = Some(after_tmp);

        let handle = thread::spawn(move ||
        {
            let mut voted = Vec::new();

            let comments = match json.find("data").and_then(|x| x.find("children")).and_then(|x| x.as_array())
            {
                Some(c) => c.to_owned(),
                None => return Err(ScrapeError::JsonError)
            };

            for comment in comments.into_iter()
            {
                if let Some(b) = comment.find("data").and_then(|x| x.find("likes")).and_then(|x| x.as_boolean())
                {
                    let id = match comment.find("data").and_then(|x| x.find("id")).and_then(|x| x.as_string())
                    {
                        Some(s) => s.to_owned(),
                        None => return Err(ScrapeError::JsonError)
                    };
                    let vote = match b
                    {
                        true => Vote::Up,
                        false => Vote::Down
                    };

                    voted.push((id, vote));
                }
            }

            Ok(voted)
        });

        threads.push(handle);

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

                let num = match string.parse::<i32>()
                {
                    Ok(n) => n,
                    Err(_) => return Err(ScrapeError::LimitHeaderError)
                };

                num
            },
            None => return Err(ScrapeError::LimitHeaderError)
        };

        if remaining < 2
        {
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

                    let num = match string.parse::<i32>()
                    {
                        Ok(n) => n,
                        Err(_) => return Err(ScrapeError::ResetHeaderError)
                    };

                    num
                },
                None => return Err(ScrapeError::ResetHeaderError)
            };

            thread::sleep(::std::time::Duration::from_secs(reset as u64));
        }

        count += 100;
    }

    let mut comments = Vec::new();

    for thread in threads.into_iter()
    {
        match thread.join()
        {
            Ok(result) => match result
            {
                Ok(mut v) => comments.append(&mut v),
                Err(e) => return Err(e)
            },
            Err(_) => return Err(ScrapeError::OtherError)
        }
    }

    Ok(comments)
}

pub fn scrape_posts(client: &hyper::Client, token: &str, user: &str) -> Result<Vec<(String, Vote)>, ScrapeError>
{
    unimplemented!()
}

#[derive(Copy, Clone)]
pub enum Vote
{
    Up,
    Down
}

#[derive(Copy, Clone)]
pub enum ScrapeError
{
    SendError,
    LimitHeaderError,
    ResetHeaderError,
    JsonError,
    OtherError,
}