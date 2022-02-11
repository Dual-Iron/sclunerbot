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

const SNAGS: &[&'static str] = &[
    "scluner",
    "sclooner",
    "schloon",
    "scloonie",
    "scloob",
    "@scluner#7833",
    // I could include `<@941409497149239396>`, but as-is there's a small chance he won't respond to pings from mobile users which I find funny
    "<@!941409497149239396>",
];

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
        let max_dist = 1 + (sq_rand() * 5.0) as usize;
        let msg_match = msg.content.to_lowercase();
        if SNAGS
            .iter()
            .any(|&snag| is_match(snag, &msg_match, max_dist))
        {
            logret!(msg.channel_id.say(ctx, get_response(ctx, msg).await).await);
        }
    }
}

fn is_match(snag: &str, substr: &str, max_dist: usize) -> bool {
    let pat = bitap::Pattern::new(snag).unwrap();
    let mut lev = pat.lev(&substr, max_dist);
    lev.next().is_some()
}

fn sq_rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0)
}

async fn get_response(ctx: &Context, msg: &Message) -> String {
    let response = QUIPS.choose(&mut thread_rng()).unwrap();
    let mut response = response
        .replace("<ping>", &msg.author.mention().to_string())
        .replace("<user>", &crate::nick(&msg.author.name, true))
        .replace("<msg>", &crate::nick(&msg.content, true))
        .replace("<emoji>", &get_emoji_txt(ctx, msg).await);
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
