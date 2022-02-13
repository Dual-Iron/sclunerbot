use crate::{logret, util::can_send};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::guild::Guild,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        if !is_new {
            println!("present in guild {} aka {}", guild.id, guild.name);
            return;
        }

        println!("just joined guild {} aka {}", guild.id, guild.name);

        // Say hello!
        for channel in guild.channels.values() {
            if can_send(&ctx, channel).await {
                let server = crate::nick(&guild.name, false);
                logret!(channel.say(&ctx, format!("hello {server}")).await);
                break;
            }
        }
    }
}
