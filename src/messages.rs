use crate::{logret, util::*};
use rand::prelude::*;
use serenity::{
    client::Context,
    framework::standard::macros::hook,
    model::{channel::Message, guild::Guild, id::ChannelId, prelude::Activity},
    prelude::Mentionable,
};

const SCLUNER_CHANNEL: ChannelId = ChannelId(941476148347551764);
const SNAGS: &[&str] = &["scluner", "@scluner#7833", "<@!941409497149239396>"];

// Separate hook so intellisense works.
#[hook]
pub async fn handle_message_hook(ctx: &Context, msg: &Message) {
    handle_message(ctx, msg).await
}

// Called when the bot receives a message that wasn't handled by a command.
pub async fn handle_message(ctx: &Context, msg: &Message) {
    // Let the owner send messages as the bot.
    if msg.guild_id.is_none() && msg.author.id == 303617148411183105 {
        if let Some((id, content)) = msg.content.split_once(' ') {
            if let Ok(id) = id.parse() {
                logret!(ChannelId(id).say(ctx, content).await);
                return;
            }
            if "status" == id {
                ctx.set_activity(Activity::watching(content)).await;
                return;
            }
        }
    }

    if let Some(guild) = msg.guild(ctx).await {
        guild_message(ctx, msg, guild).await
    } else {
        dm(ctx, msg).await
    }
}

async fn dm(ctx: &Context, msg: &Message) {
    // Get message content with attachment URLs tagged on.
    let mut msg_content = msg.content_safe(ctx).await;
    for attachment in &msg.attachments {
        msg_content.push('\n');
        msg_content += &attachment.url;
    }
    // Can't send a message with empty content.
    if msg_content.is_empty() {
        logret!(msg.reply(ctx, crate::lang::lacks_text()).await);
        return;
    }
    // Get one of the latest 200 messages in SCLUNER_CHANNEL so we can reply with it.
    match SCLUNER_CHANNEL.messages(ctx, |g| g.limit(200)).await {
        Ok(messages) => {
            let chosen = logret!(messages.choose(&mut rand::thread_rng()).ok_or("how"));
            let chosen_content = chosen.content_safe(ctx).await;
            logret!(msg.channel_id.say(ctx, chosen_content).await);
            logret!(SCLUNER_CHANNEL.say(ctx, msg_content).await);
        }
        Err(e) => {
            dbg!(e);
            logret!(msg.reply(ctx, "couldn't fetch from scluner channel").await);
        }
    }
}

async fn guild_message(ctx: &Context, msg: &Message, guild: Guild) {
    // If you can send messages in the channel, maybe send a quip.
    if let Some(channel) = guild.channels.get(&msg.channel_id) {
        if can_send(ctx, channel).await {
            let text = msg.content.to_lowercase();
            if SNAGS.iter().any(|&s| is_match(s, &text)) {
                logret!(msg.channel_id.say(ctx, get_response(ctx, msg).await).await);
            }
        }
    }
}

fn is_match(substr: &str, text: &str) -> bool {
    let rand = rand::thread_rng().gen_range(0.0..1.0);

    text.split(|c: char| !c.is_alphanumeric()).any(|s| {
        let dist = strsim::normalized_damerau_levenshtein(s, substr);

        dist * dist * dist > rand
    })
}

async fn get_response(ctx: &Context, msg: &Message) -> String {
    let response = crate::lang::quip();
    let mut response = response
        .replace("<ping>", &msg.author.mention().to_string())
        .replace("<user>", &crate::nick(&msg.author.name, true))
        .replace("<msg>", &crate::nick(&msg.content, true))
        .replace("<emoji>", &get_emoji_txt(ctx, msg.guild(ctx).await).await)
        .replace("<emoji2>", &get_emoji_txt(ctx, msg.guild(ctx).await).await);
    let screaming_text =
        msg.content.chars().filter(|c| c.is_uppercase()).count() > msg.content.len() / 2;
    if screaming_text {
        response.make_ascii_uppercase();
    }
    response
}
