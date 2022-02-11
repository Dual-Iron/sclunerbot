use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};

mod commands;

#[group]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

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
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("error creating client");

    println!("scluner lives again");

    // Start the server's listening loop.
    client.start().await.unwrap();
}

// Quick macro to log errors.
macro_rules! dbge {
    ($e:expr) => {
        if let Err(e) = $e {
            use std::time::SystemTime;
            dbg!(SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap());
            dbg!(e.to_string());
        }
    };
}
pub(crate) use dbge;
