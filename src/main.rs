mod commands;
mod data;
mod utils;
mod mongo;
mod metrics;

use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude::GatewayIntents;
use serenity::client::Context;
use serenity::http::GuildPagination;
use serenity::model::id::ChannelId;
use serenity::model::prelude::Activity;
use crate::commands::events::create_event_embeds;
use crate::data::{Data};
use crate::metrics::core::{AllRegistries, create_registries, Empty, setup_server};
use crate::mongo::core::get_all_status_messages;

fn update_daily(ctx: Context, all_registries: Arc<AllRegistries>) {
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60*60));
        loop {
            get_total_guilds_count(&ctx, &all_registries).await;
            let status_messages = get_all_status_messages().await;
            let embeds = create_event_embeds().await;

            for sm in status_messages {
                let ctx = ctx.clone();
                let embeds = embeds.clone();
                tokio::spawn(async move {
                    if sm.channel_id == 0 {return}
                    let msg = ChannelId::from(sm.channel_id as u64).message(ctx.clone().http, sm.message_id as u64).await;
                    match msg {
                        Ok (mut m) => {
                            match m.edit(&ctx.http, |f| {
                                f.set_embeds(embeds)
                            }).await {
                                Ok(..) => {},
                                Err(e) => println!("Error while editing message {}", e)
                            }
                        },
                        Err(_) => {println!("Cannot update guild : {}", sm.channel_id);}
                    }
                });
            }
            interval.tick().await;
        }
    });
}

async fn get_total_guilds_count(ctx: &Context, all_registries: &Arc<AllRegistries>) {
    let mut cnt: i64 = 0;
    let mut guilds = ctx.http.get_guilds(None, None).await.expect("Cannot get guilds");
    while !guilds.is_empty() {
        cnt += guilds.len() as i64;
        guilds = ctx.http.get_guilds(Some(&GuildPagination::After(guilds.last().expect("Cannot get last").id)), None).await.expect("Cannot get nexts");
    }
    all_registries.server_count.get_or_create(&Empty {}).set(cnt);
}

#[tokio::main]
async fn main() {
    // Build our client
    let framework = poise::Framework::builder()
        .token(std::env::var("TOKEN").expect("No token found "))
        .intents(GatewayIntents::non_privileged())
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::character::character(),
                commands::events::create_event_message()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(Activity::listening("/character")).await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let all_registries = Arc::new(create_registries());


                let server_registry = Arc::clone(&all_registries);
                tokio::task::spawn(async move {
                                       setup_server(server_registry).await
                                   });

                let updater_registry = Arc::clone(&all_registries);
                update_daily(ctx.clone(), updater_registry);


                println!("Bot Started");
                Ok(Data {registries: all_registries})
            })
        });

    framework.run_autosharded().await.unwrap();

}