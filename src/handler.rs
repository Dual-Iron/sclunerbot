use crate::logret;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{guild::Guild, id::UserId},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        if !is_new {
            return;
        }

        // Tell the owner that the bot just joined another server
        if let Ok(user) = UserId(303617148411183105).to_user(&ctx).await {
            let content = format!("just joined guild {} aka {}", guild.id, guild.name);
            if let Err(e) = user.dm(&ctx, |m| m.content(content)).await {
                dbg!(e);
            }
        }

        // Say hello!
        for channel in guild.channels.values() {
            if channel.is_text_based() {
                let me = ctx.cache.current_user().await;
                let perms = logret!(channel.permissions_for_user(&ctx, me).await);
                if perms.send_messages() {
                    let server = crate::nick(&guild.name, false);
                    logret!(channel.say(&ctx, format!("hello {server}")).await);
                    break;
                }
            }
        }
    }
}
