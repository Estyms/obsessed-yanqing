mod commands;
mod data;
mod utils;
use poise::serenity_prelude::GatewayIntents;
use crate::data::Data;

#[tokio::main]
async fn main() {
    // Build our client.
    let framework = poise::Framework::builder()
        .token(std::env::var("TOKEN").expect("No token found"))
        .intents(GatewayIntents::non_privileged())
        .options(poise::FrameworkOptions {
            commands: vec![
                 commands::character::character()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}