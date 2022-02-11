#![feature(string_remove_matches)]

use serenity::client::Client;
use serenity::framework::standard::{macros::group, StandardFramework};

mod commands;
mod handler;

#[group]
struct General;

#[tokio::main]
async fn main() {
    // Accept ~ and pings for prefixes.
    let framework = StandardFramework::new()
        .configure(|c| c.prefixes(vec!["~", "<@!941409497149239396> "]))
        .normal_message(commands::messages)
        .group(&GENERAL_GROUP);

    // Log in using a bot token provided by an environment variable.
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN environment variable");
    let mut client = Client::builder(token)
        .event_handler(handler::Handler)
        .framework(framework)
        .await
        .expect("error creating client");

    println!("scluner lives again");

    // Start the server's listening loop.
    client.start().await.unwrap();
}

// Gets a scluner-friendly name
pub fn nick(name: &str, cull: bool) -> String {
    // This function could just take a `&mut String` but I'm lazy
    let mut name = name.to_string();
    if cull {
        let whitespace_pos = name.split_whitespace().next().unwrap_or(&name).len();
        name.truncate(whitespace_pos);
    }
    name.remove_matches(|c: char| !c.is_alphabetic());
    name.make_ascii_lowercase();
    name
}

// If there are errors, log them, then return from the function.
macro_rules! logret {
    ($e:expr) => {
        match $e {
            Ok(o) => o,
            Err(e) => {
                let time = chrono::Local::now().format("%X %v");
                println!("{time:25}{e}");
                dbg!(e);
                println!();
                return;
            }
        }
    };
}
pub(crate) use logret;
