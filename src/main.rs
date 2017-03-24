extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate url;
extern crate clap;

mod login;
mod vote;
mod comment;
mod post;
mod scrape_error;

use std::io;

fn main()
{
    let matches = clap::App::new("user_vote_scraper")
        .about("Search reddit users posts and comment for items that you have voted on")
        .arg(clap::Arg::with_name("username")
            .short("u")
            .long("username")
            .help("the user to search")
            .index(1)
            .takes_value(true)
            .required(true))
        .arg(clap::Arg::with_name("comments")
            .short("c")
            .long("comments")
            .help("search comments"))
        .arg(clap::Arg::with_name("posts")
            .short("p")
            .long("posts")
            .help("search posts"))
        .arg(clap::Arg::with_name("metrics")
            .short("m")
            .long("metrics")
            .help("prefix any output with metrics from the searched items"))
        .arg(clap::Arg::with_name("output")
            .short("o")
            .long("output")
            .help("the output type: links = list of links, ids = list of ids, api = use reddit's api to generate the list, none = no output")
            .takes_value(true)
            .default_value("none")
            .validator(|s| match s.as_str()
            {
                "links" | "ids" | "api" | "none" => Ok(()),
                _ => Err("the only valid values are 'links', 'ids', 'api', and 'none'".to_owned())
            }))
        .arg(clap::Arg::with_name("format")
            .short("f")
            .long("format")
            .help("the output format: list = newline separated list, csv = comma separated list")
            .takes_value(true)
            .default_value("list")
            .validator(|s| match s.as_str()
            {
                "list" | "csv" => Ok(()),
                _ => Err("the only valid valuds are 'list' and 'csv'".to_owned())
            }))
        .arg(clap::Arg::with_name("code")
            .long("code")
            .help("if you already have the code from the OAuth2 redirect, this will skip the login process")
            .takes_value(true))
        .get_matches();
}