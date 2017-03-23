use url;
use hyper;
use rustc_serialize;

use std::io;
use std::io::Read;

pub fn get_code() -> Result<String, CodeError>
{
    let oauth_url = format!("https://www.reddit.com/api/v1/authorize\
        ?client_id={client_id}\
        &response_type={response_type}\
        &state={state}\
        &redirect_uri={uri}\
        &duration={duration}\
        &scope={scope}",
        client_id = "DeUMM_qG5zG41Q",
        response_type = "code",
        state = "makethisrandomlater",
        uri = "https://matthew-maclean.github.io/user_vote_scraper/",
        duration = "temporary",
        scope = "history");
    
    println!("please open this URL in your browser:\n\n    {}\n", oauth_url);
    println!("when you have been redirected, please paste the redirect url here:");

    let input =
    {
        let mut s = String::new();
        match io::stdin().read_line(&mut s)
        {
            Ok(_) => s,
            Err(_) => return Err(CodeError::InputError)
        }
    };

    let (err, code) =
    {
        let mut e = None;
        let mut c = None;
        for pair in match url::Url::parse(&input)
        {
            Ok(u) => u,
            Err(_) => return Err(CodeError::UrlParseError)
        }.query_pairs().into_owned()
        {
            if pair.0 == "code"
            {
                c = Some((pair.1).clone())
            }
            else if pair.0 == "error"
            {
                e = Some((pair.1).clone())
            }
        }
        (e, c)
    };

    if let Some(s) = err
    {
        if s == "access_denied"
        {
            return Err(CodeError::AccessDenied)
        }
        else
        {
            return Err(CodeError::OtherAuthError)
        }
    }

    if let Some(s) = code
    {
        return Ok(s)
    }
    else
    {
        return Err(CodeError::NoCodeInUrl)
    }
}

pub fn get_token(code: &str, client: &hyper::Client) -> Result<String, TokenError>
{
    let post_url = "https://www.reddit.com/api/v1/access_token";
    let post_body = format!("grant_type={grant_type}\
        &code={code}\
        &redirect_uri={uri}",
        grant_type = "authorization_code",
        code = code,
        uri = "https://matthew-maclean.github.io/user_vote_scraper/");
    
    let header =
    {
        let mut h = hyper::header::Headers::new();
        h.set(
            hyper::header::Authorization(
                hyper::header::Basic
                {
                    username: "DeUMM_qG5zG41Q".to_owned(),
                    password: Some("".to_owned())
                }
            )
        );
        h
    };
    
    let mut res = match client.post(post_url).headers(header).body(&post_body).send()
    {
        Ok(r) => r,
        Err(_) => return Err(TokenError::SendError)
    };

    if res.status == hyper::status::StatusCode::Unauthorized
    {
        return Err(TokenError::HeaderError)
    }

    let json =
    {
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        match rustc_serialize::json::Json::from_str(&body)
        {
            Ok(j) => j,
            Err(_) => return Err(TokenError::OtherError)
        }
    };

    match json.find("access_token")
    {
        Some(token) => match token.as_string()
        {
            Some(s) => Ok(s.to_owned()),
            None => return Err(TokenError::OtherError)
        },
        None => return Err(TokenError::OtherError)
    }
}

use std::fmt;

#[derive(Copy, Clone)]
pub enum CodeError
{
    InputError,
    UrlParseError,
    NoCodeInUrl,
    AccessDenied,
    OtherAuthError,
}

impl fmt::Display for CodeError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            &CodeError::InputError => write!(f, "Error recieving input from console"),
            &CodeError::UrlParseError => write!(f, "Unable to parse the provided URL"),
            &CodeError::NoCodeInUrl => write!(f, "The provided URL does not contain a 'code' parameter"),
            &CodeError::AccessDenied => write!(f, "The user did not allow this application access"),
            &CodeError::OtherAuthError => write!(f, "Another error has occured (probably not your fault)")
        }
    }
}

#[derive(Copy, Clone)]
pub enum TokenError
{
    SendError,
    HeaderError,
    OtherError,
}

impl fmt::Display for TokenError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            &TokenError::SendError => write!(f, "An error occured while sending the token request"),
            &TokenError::HeaderError => write!(f, "There was an error with the POST header (not your fault)"),
            &TokenError::OtherError => write!(f, "Another type of error occured")
        }
    }
}