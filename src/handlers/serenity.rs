use std::env;

use log::{debug, error, info};
use serenity::all::{GuildId, Interaction};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::gateway::ActivityData;
use serenity::async_trait;
use crate::commands;

use super::SerenityHandler;

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}!", ready.user.tag());
        ctx.set_activity(Some(ActivityData::watching("Azumanga Daioh")));

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an int"),
        );

        let _ = guild_id.set_commands(&ctx.http, vec![
            commands::hello::register(),
        ]).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("Received command interaction: {command:#?}");
    
            let content = match command.data.name.as_str() {
                "hello" => Some(commands::hello::run(&command.data.options())),
                _ => Some("Not implemented".to_string()),
            };
    
            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(err) = command.create_response(&ctx.http, builder).await {
                    error!("Failed to reply to slash command: {err}");
                }
            }
        }
    }
}