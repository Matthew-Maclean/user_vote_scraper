extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate url;

mod login;
mod vote;
mod comment;
mod post;
mod scrape_error;

use std::io;

fn main()
{
    println!("Whose posts would you like to scrape?");
    let user =
    {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_owned();
        s
    };

    let code = match login::get_code()
    {
        Ok(c) => c,
        Err(e) =>
        {
            println!("Error: {}", e);
            return;
        }
    };

    let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
    let con = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(con);

    let token = match login::get_token(&code, &client)
    {
        Ok(t) => t,
        Err(e) =>
        {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Enter a limit (or leave blank for no limit):");
    let limit =
    {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim()
        {
            "" => ::std::i32::MAX,
            i => match i.parse::<i32>()
            {
                Ok(n) => n,
                Err(_) =>
                {
                    println!("Error: could not parse limit");
                    return;
                }
            }
        }
    };

    println!("Scrape comments ('c'), posts ('p'), or both ('b', or blank)?");
    let input =
    {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_owned();
        s
    };

    if input == "c"
    {
        println!("Scraping comments, this may take some time.");
        let comments = match comment::Comment::scrape(&client, &token, &user, limit)
        {
            Ok(c) => c,
            Err(e) =>
            {
                println!("Error: {}", e);
                return;
            }
        };

        println!("Found {} comments that you voted on", comments.len());
        for comment in comments.into_iter()
        {
            println!("{}", comment);
        }
    }
    else if input == "p"
    {
        println!("Scraping posts, this may take some time.");
        let posts = match post::Post::scrape(&client, &token, &user, limit)
        {
            Ok(c) => c,
            Err(e) =>
            {
                println!("Error: {}", e);
                return;
            }
        };

        println!("Found {} posts that you voted on", posts.len());
        for post in posts.into_iter()
        {
            println!("{}", post);
        }
    }
    else
    {
        println!("Scraping comments, this may take some time.");
        let comments = match comment::Comment::scrape(&client, &token, &user, limit)
        {
            Ok(c) => c,
            Err(e) =>
            {
                println!("Error: {}", e);
                return;
            }
        };

        println!("Found {} comments that you voted on", comments.len());
        println!("Scraping posts, this may take some time.");
        let posts = match post::Post::scrape(&client, &token, &user, limit)
        {
            Ok(c) => c,
            Err(e) =>
            {
                println!("Error: {}", e);
                return;
            }
        };

        println!("Found {} posts that you voted on", posts.len());
        println!("posts:");
        for post in posts.into_iter()
        {
            println!("{}", post);
        }
        println!("comments:");
        for comment in comments.into_iter()
        {
            println!("{}", comment);
        }
    }
}