use crate::dbge;
use rand::{prelude::*, thread_rng};
use serenity::{
    client::Context,
    framework::standard::macros::hook,
    model::{channel::Message, id::ChannelId},
    prelude::Mentionable,
};

lazy_static::lazy_static! {
    static ref QUIPS: Vec<&'static str> = {
        include_str!("../quips.txt").lines().collect()
    };
}

static SNAGS: &[&'static str] = &[
    "scluner",
    "sclooner",
    "schloon",
    "@scluner#7833",
    "<@!941409497149239396>",
];

// Called when the bot receives a message.
#[hook]
pub async fn messages(ctx: &Context, msg: &Message) {
    // If the bot owner DMs the bot with `[channel_id] [content]`, make the bot send [content] in that channel.
    if msg.guild_id.is_none() && msg.author.id == 303617148411183105 {
        if let Some((id, content)) = msg.content.split_once(' ') {
            if let Ok(id) = id.parse() {
                dbge!(ChannelId(id).say(&ctx.http, content).await);
                return;
            }
        }
    }

    // Otherwise, send the message to some hardcoded channel.
    if msg.guild_id.is_none() {
        let reply = ChannelId(941476148347551764)
            .say(&ctx.http, msg.content_safe(ctx).await)
            .await;
        if let Err(e) = reply {
            dbge!(msg.reply(&ctx.http, e.to_string()).await);
        }
    } else {
        let max_rand = rand::thread_rng().gen_range(0..=7);
        if SNAGS
            .iter()
            .any(|&s| strsim::damerau_levenshtein(s, &msg.content) < max_rand)
        {
            let response = QUIPS.choose(&mut thread_rng()).unwrap();
            let response = response
                .replace("<ping>", &msg.author.mention().to_string())
                .replace("<user>", &get_nick(ctx, msg).await);
            dbge!(msg.channel_id.say(&ctx.http, response).await);
        }
    }
}

async fn get_nick(ctx: &Context, msg: &Message) -> String {
    let mut raw = msg
        .author
        .nick_in(ctx, msg.guild_id.unwrap())
        .await
        .unwrap_or_else(|| msg.author.name.clone());

    let whitespace_pos = raw.split_whitespace().next().unwrap_or(&raw).len();
    raw.truncate(whitespace_pos);
    raw.make_ascii_lowercase();
    raw
}
