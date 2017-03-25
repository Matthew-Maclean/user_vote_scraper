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
mod comment;
mod post;
mod scrape_error;

fn main() {
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
        .arg(clap::Arg::with_name("limit")
            .short("l")
            .long("limit")
            .help("the maximum number of posts or comments to search")
            .takes_value(true)
            .default_value("2147483647") // i32 max value
            .validator(|s| match s.parse::<i32>()
            {
                Ok(_) => Ok(()),
                Err(_) => Err("limit needs to be a valid integer".to_owned())
            }))
        .arg(clap::Arg::with_name("output")
            .short("o")
            .long("output")
            .help("the output type: links = list of links, ids = list of ids, api = use \
                   reddit's api to generate the some links, none = no output")
            .takes_value(true)
            .default_value("none")
            .validator(|s| match s.as_str() {
                "links" | "ids" | "api" | "none" => Ok(()),
                _ => Err("the only valid values are 'links', 'ids', 'api', and 'none'".to_owned()),
            }))
        .arg(clap::Arg::with_name("code")
            .long("code")
            .help("if you already have the code from the OAuth2 redirect, this will skip the \
                   login process")
            .takes_value(true))
        .get_matches();

    let user = matches.value_of("username").unwrap().to_owned();
    let limit = matches.value_of("limit").unwrap().to_owned().parse::<i32>().unwrap();
    let code = if let Some(c) = matches.value_of("code") {
        c.to_owned()
    } else {
        match login::get_code() {
            Ok(c) => c,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
    };

    let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
    let con = hyper::net::HttpsConnector::new(ssl);
    let client = hyper::Client::with_connector(con);

    let token = match login::get_token(&code, &client) {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let comments = if matches.is_present("comments") {
        match comment::Comment::scrape(&client, &token, &user, limit) {
            Ok(c) => Some(c),
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
    } else {
        None
    };

    let posts = if matches.is_present("posts") {
        match post::Post::scrape(&client, &token, &user, limit) {
            Ok(p) => Some(p),
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }
    } else {
        None
    };

    let output = match matches.value_of("output").unwrap() {
        "links" => {
            let mut o = Vec::new();
            if let &Some(ref c) = &comments {
                o.append(&mut c.iter().map(comment::Comment::to_string).collect());
            }
            if let &Some(ref p) = &posts {
                o.append(&mut p.iter().map(|x| x.permalink.clone()).collect());
            }
            o
        }
        "ids" => {
            let mut o = Vec::new();
            if let &Some(ref c) = &comments {
                o.append(&mut c.iter().map(|x| x.id.to_string()).collect());
            }
            if let &Some(ref p) = &posts {
                o.append(&mut p.iter().map(|x| x.id.to_string()).collect());
            }
            o
        }
        "api" => {
            let mut o = Vec::new();
            if let &Some(ref c) = &comments {
                o.append(&mut generate_comment_api_links(c));
            }
            if let &Some(ref p) = &posts {
                o.append(&mut generate_post_api_links(p));
            }
            o
        }
        "none" => Vec::new(),
        _ => {
            println!("Error: invalid output format");
            return;
        }
    };

    if matches.is_present("metrics") {
        if let Some(c) = comments {
            println!("{}", generate_comment_metrics(&c));
        }
        if let Some(p) = posts {
            println!("{}", generate_post_metrics(&p));
        }
    }

    println!("{}", output.join("\n"));
}

fn generate_comment_api_links(comments: &Vec<comment::Comment>) -> Vec<String> {
    let lists = if comments.len() <= 100 {
        vec![comments.iter().map(|x| format!("t1_{}", x.id)).collect()]
    } else {
        let mut v = vec![Vec::new()];
        let mut acc = 0;
        let mut ind = 0;
        for comment in comments.iter().map(|x| format!("t1_{}", x.id)) {
            v[ind].push(comment);
            acc += 1;
            if acc % 100 == 0 {
                ind += 1;
                v.push(Vec::new());
            }
        }
        v
    };

    lists.into_iter().map(|x| format!("https://reddit.com/api/info?id={}", x.join(","))).collect()
}

fn generate_post_api_links(posts: &Vec<post::Post>) -> Vec<String> {
    let lists = if posts.len() <= 100 {
        vec![posts.iter().map(|x| format!("t3_{}", x.id)).collect()]
    } else {
        let mut v = vec![Vec::new()];
        let mut acc = 0;
        let mut ind = 0;
        for post in posts.iter().map(|x| format!("t3_{}", x.id)) {
            v[ind].push(post);
            acc += 1;
            if acc % 100 == 0 {
                ind += 1;
                v.push(Vec::new());
            }
        }
        v
    };

    lists.into_iter().map(|x| format!("https://reddit.com/api/info?id={}", x.join(","))).collect()
}

fn generate_comment_metrics(comments: &Vec<comment::Comment>) -> String {
    let mut ups = 0;
    let mut downs = 0;

    for comment in comments.iter() {
        match comment.likes {
            Some(true) => ups += 1,
            Some(false) => downs += 1,
            None => {}
        }
    }

    format!("Of {} comments, you upvoted {} ({:.2}%), and downvoted {} ({:.2}%)",
            comments.len(),
            ups,
            ups as f64 / comments.len() as f64,
            downs,
            downs as f64 / comments.len() as f64)
}

fn generate_post_metrics(posts: &Vec<post::Post>) -> String {
    let mut ups = 0;
    let mut downs = 0;

    for post in posts.iter() {
        match post.likes {
            Some(true) => ups += 1,
            Some(false) => downs += 1,
            None => {}
        }
    }

    format!("Of {} posts, you upvoted {} ({:.2}%), and downvoted {} ({:.2}%)",
            posts.len(),
            ups,
            ups as f64 / posts.len() as f64,
            downs,
            downs as f64 / posts.len() as f64)
}
