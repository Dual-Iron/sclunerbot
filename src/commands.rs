use crate::logret;
use rand::{prelude::*, thread_rng};
use serenity::{
    client::Context,
    framework::standard::macros::hook,
    model::{channel::Message, guild::Emoji, id::ChannelId},
    prelude::Mentionable,
};

lazy_static::lazy_static! {
    static ref QUIPS: Vec<&'static str> = {
        include_str!("../quips.txt").lines().collect()
    };
}

const SNAGS: &[&'static str] = &["scluner", "@scluner#7833", "<@!941409497149239396>"];

// Called when the bot receives a message.
#[hook]
pub async fn messages(ctx: &Context, msg: &Message) {
    // Let the owner send messages as the bot.
    if msg.guild_id.is_none() && msg.author.id == 303617148411183105 {
        if let Some((id, content)) = msg.content.split_once(' ') {
            if let Ok(id) = id.parse() {
                logret!(ChannelId(id).say(ctx, content).await);
                return;
            }
        }
    }

    // If it's another DM, send the message to some hardcoded channel.
    if msg.guild_id.is_none() {
        let reply = ChannelId(941476148347551764)
            .say(ctx, msg.content_safe(ctx).await)
            .await;
        if let Err(e) = reply {
            logret!(msg.reply(ctx, e.to_string()).await);
        }
    }
    // Otherwise, listen in for potential quips.
    else {
        let text = msg.content.to_lowercase();
        if SNAGS.iter().any(|&s| is_match(s, &text)) {
            logret!(msg.channel_id.say(ctx, get_response(ctx, msg).await).await);
        }
    }
}

fn is_match(substr: &str, text: &str) -> bool {
    let rand = rand::thread_rng().gen_range(0.0..1.0);

    text.split(|c: char| !c.is_alphanumeric()).any(|s| {
        let dist = strsim::normalized_damerau_levenshtein(s, substr);

        dist * dist > rand
    })
}

async fn get_response(ctx: &Context, msg: &Message) -> String {
    let response = QUIPS.choose(&mut thread_rng()).unwrap();
    let mut response = response
        .replace("<ping>", &msg.author.mention().to_string())
        .replace("<user>", &crate::nick(&msg.author.name, true))
        .replace("<msg>", &crate::nick(&msg.content, true))
        .replace("<emoji>", &get_emoji_txt(ctx, msg).await)
        .replace("<emoji2>", &get_emoji_txt(ctx, msg).await);
    let screaming_text =
        msg.content.chars().filter(|c| c.is_uppercase()).count() > msg.content.len() / 2;
    if screaming_text {
        response.make_ascii_uppercase();
    }
    response
}

async fn get_emoji_txt(ctx: &Context, msg: &Message) -> String {
    match get_emojis(ctx, msg).await.choose(&mut thread_rng()) {
        Some(e) => e.to_string(),
        None => ":slight_smile:".to_string(),
    }
}

async fn get_emojis(ctx: &Context, msg: &Message) -> Vec<Emoji> {
    match msg.guild(ctx).await {
        Some(g) => g.emojis(ctx).await.unwrap_or_default(),
        None => Vec::new(),
    }
}
