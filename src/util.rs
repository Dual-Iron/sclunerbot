use rand::{prelude::SliceRandom, thread_rng};
use serenity::{
    client::Context,
    model::{
        channel::GuildChannel,
        guild::{Emoji, Guild},
    },
};

pub async fn can_send(ctx: &Context, channel: &GuildChannel) -> bool {
    channel.is_text_based()
        && channel
            .permissions_for_user(ctx, ctx.cache.current_user().await)
            .await
            .map(|p| p.send_messages() && p.read_messages())
            .unwrap_or_default()
}

pub async fn get_emoji_txt(ctx: &Context, guild: Option<Guild>) -> String {
    match get_emojis(ctx, guild).await.choose(&mut thread_rng()) {
        Some(e) => e.to_string(),
        None => ":slight_smile:".to_string(),
    }
}

pub async fn get_emojis(ctx: &Context, guild: Option<Guild>) -> Vec<Emoji> {
    match guild {
        Some(g) => g.emojis(ctx).await.unwrap_or_default(),
        None => Vec::new(),
    }
}
