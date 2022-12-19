use crate::{logret, util::*};
use rand::prelude::*;
use serenity::{
    builder::GetMessages,
    client::Context,
    framework::standard::macros::hook,
    model::{channel::Message, guild::Guild, id::ChannelId, prelude::Activity},
    prelude::Mentionable,
};

const SCLUNER_CHANNEL: ChannelId = ChannelId(941476148347551764);

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
        if guild.name == "h(orse) h(ouse)" {
            dm(ctx, msg).await
        } else {
            guild_message(ctx, msg, guild).await
        }
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
    match SCLUNER_CHANNEL.messages(ctx, choose_messages).await {
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

fn choose_messages(g: &mut GetMessages) -> &mut GetMessages {
    let rand = thread_rng().gen_range(0..10);
    match rand {
        0 => g.around(1053756384891654154),
        1 => g.around(1053405728192352329),
        2 => g.around(1051937183830908938),
        3 => g.around(1049404756864540722),
        4 => g.around(1048501031484526733),
        5 => g.around(1047745893916495892),
        6 => g.around(1047737192019152936),
        7 => g.around(1047727018743189574),
        8 => g.around(1047733482627006484),
        _ => g,
    }
    .limit(200)
}

async fn guild_message(ctx: &Context, msg: &Message, guild: Guild) {
    // If you can send messages in the channel, maybe send a quip.
    if let Some(channel) = guild.channels.get(&msg.channel_id) {
        if can_send(ctx, channel).await {
            let text = msg.content.to_lowercase();
            let should_reply = should_reply(ctx, msg, &text).await;
            if should_reply {
                logret!(msg.channel_id.say(ctx, get_response(ctx, msg).await).await);
            }
        }
    }
}

async fn should_reply(ctx: &Context, msg: &Message, text: &str) -> bool {
    if let Some(m) = &msg.referenced_message {
        // If replying to scluner, it should respond
        m.author.id == ctx.cache.current_user().await.id
    } else {
        // Respond to direct pings and, sometimes, when it mistakes someone saying "scluner"
        msg.mentions_me(ctx).await.unwrap_or(false) || is_match(text, "scluner")
    }
}

fn is_match(text: &str, substr: &str) -> bool {
    let rand = thread_rng().gen_range(0.0..1.0);

    text.split(|c: char| !c.is_alphanumeric()).any(|s| {
        let dist = strsim::normalized_damerau_levenshtein(s, substr);

        dist.powf(2.2) > rand
    })
}

async fn get_response(ctx: &Context, msg: &Message) -> String {
    let mut response = crate::lang::quip().to_string();

    // Scream if they screamed. Make sure to exclude the special parts inside `<>`
    let screaming_text =
        msg.content.chars().filter(|c| c.is_uppercase()).count() > msg.content.len() / 2;
    if screaming_text {
        let mut response_screaming = String::with_capacity(response.len());
        let mut special_part = false;
        for c in response.chars() {
            if c == '<' {
                special_part = true;
            } else if c == '>' {
                special_part = false;
            }

            if special_part {
                response_screaming.push(c);
            } else {
                response_screaming.push(c.to_ascii_uppercase());
            }
        }
        response = response_screaming;
    }

    // Replace special parts with the relevant text
    response
        .replace("<ping>", &msg.author.mention().to_string())
        .replace("<user>", &crate::nick(&msg.author.name, true))
        .replace("<msg>", &random_word(&msg.content))
        .replace("<emoji>", &get_emoji_txt(ctx, msg.guild(ctx).await).await)
        .replace("<emoji2>", &get_emoji_txt(ctx, msg.guild(ctx).await).await)
}

fn random_word(msg: &str) -> String {
    let common_words = &crate::lang::COMMON_WORDS;
    let len_range = 1..thread_rng().gen_range(6..=14);

    msg.split_whitespace()
        .map(|s| crate::nick(s, false))
        .filter(|s| len_range.contains(&s.len()) && !common_words.contains(&s.as_str()))
        .choose(&mut thread_rng())
        .unwrap_or("that".into())
}
