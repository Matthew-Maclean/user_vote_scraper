use url;

use std::io;

pub fn login() -> Result<String, LoginError>
{
    let oauth_url = format!("https://www.reddit.com/api/v1/authorize\
        ?client_id={client_id}\
        &response_type={response_type}\
        &state={state}
        &redirect_uri={uri}\
        &duration={duration}\
        &scope={scope}",
        client_id = "DeUMM_qG5zG41Q",
        response_type = "code",
        state = "makethisrandomlater",
        uri = "https://matthew-maclean.github.io/user_vote_scraper/",
        duration = "temporary",
        scope = "history");
    
    println!("please open this URL in your browser: {}", oauth_url);

    let input =
    {
        let mut s = String::new();
        match io::stdin().read_line(&mut s)
        {
            Ok(_) => s,
            Err(_) => return Err(LoginError::InputError)
        }
    };

    let code =
    {
        let mut c = None;
        for pair in match url::Url::parse(&input)
        {
            Ok(u) => u,
            Err(_) => return Err(LoginError::UrlParseError)
        }.query_pairs().into_owned()
        {
            if pair.0 == "code"
            {
                c = Some((pair.1).clone())
            }
        }
        c
    };

    if let Some(s) = code
    {
        return Ok(s)
    }
    else
    {
        return Err(LoginError::NoCodeInUrl)
    }
}

pub enum LoginError
{
    InputError,
    UrlParseError,
    NoCodeInUrl,
}