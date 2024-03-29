use std::sync::Arc;
use std::time::Duration;
use poise::ReplyHandle;
use serenity::builder::{CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter};
use serenity::futures::future::join_all;
use serenity::model::application::component::ButtonStyle;
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use crate::data::{Context, Error};
use crate::data::allcharacters::{Characters, get_nearest_characters};
use crate::data::character::{Character, get_character_data};
use crate::data::cones::get_light_cone;
use crate::data::description::get_all_texts;
use crate::data::proscons::{format_proscons};
use crate::metrics::characters::CharacterLabels;
use crate::utils::color_manager::get_element_color;
use crate::utils::emote_manager::{get_element_emote, get_path_emote};

enum CharacterTab {
    Home,
    Review,
    Gear(usize),
    Team
}

fn get_base_character_embed(character: &Character) -> CreateEmbed {
    CreateEmbed::default()
        .title(format!("{} {} {}", get_element_emote(&character.element), character.name, get_path_emote(&character.path)))
        .set_footer(CreateEmbedFooter::default().text("Data from https://www.prydwen.gg/").to_owned())
        .description(format!("{}\n{}\n",
                             ":star:".repeat(character.rarity.parse().unwrap_or(4)),
                             character.default_role))
        .color(get_element_color(&character.element))
        .thumbnail(format!("https://www.prydwen.gg{}", character.small_image.local_file.child_image_sharp.gatsby_image_data.images.fallback.src))
        .to_owned()
}

async fn create_menu(ctx: Context<'_>, chars: Vec<Characters>) -> ReplyHandle {
    ctx.send(|a| {
        a.content("Select the right character").components(|c| {
            c.create_action_row(|row| {
                row.create_select_menu(|menu| {
                    menu.custom_id("select_character");
                    menu.placeholder("Character");
                    menu.options(|f| {
                        chars.into_iter().for_each(|char| {
                            f.create_option(|o| o.label(char.name).value(char.slug));
                        });
                        f
                    })
                })
            })
        }).ephemeral(true)
    }).await.unwrap()
}

async fn get_character_home(character: &Character) -> Option<CreateEmbed> {
    let desc_data = &character.description;
    let desc = match desc_data {
        None => { "".to_string() }
        Some(data) => {
            let all_texts: Vec<String> = get_all_texts(&data).unwrap();
            let a = all_texts.get(0).cloned();
            match a {
                None => { "".to_string() }
                Some(b) => { b }
            }
        }
    };
    Some(
        get_base_character_embed(character)
            .description(format!("{}\n{}\n\n{}",
                                 ":star:".repeat(character.rarity.parse().unwrap_or(4)),
                                 character.default_role,
                                 desc))
            .to_owned()
    )
}

async fn get_character_review(character: &Character) -> Option<CreateEmbed> {
    Some(
        get_base_character_embed(character)
            .field(":green_circle: Pros", format_proscons( &character.pros), false)
            .field(":red_circle: Cons", format_proscons( &character.cons), false)
            .to_owned()
    )
}

async fn get_character_build(character: &Character, index: usize) -> Option<CreateEmbed> {
    let chosen_build = character.build_data.as_ref().expect("No builds").get(index).expect("Build doesnt exist");

    let clone = chosen_build.clone();
    let light_cones = join_all(clone.cones.into_iter().map(|c| async { (c.super_field ,get_light_cone(c.cone).await.expect("Cone"))})).await;

    Some(
        get_base_character_embed(character)
            .field(chosen_build.name.as_str(), chosen_build.comments.as_str(), false)


            // STUFF
            .field("Relic Sets", chosen_build.relics.iter().enumerate().map(|(i, f)| match f.relic_2.len() {
                0 => format!("{}. {}", i+1, f.relic),
                _ => format!("{}. {} + {}", i+1, f.relic, f.relic_2)
            }).collect::<Vec<String>>().join("\n"), false)

            .field("Planetary Sets", chosen_build.planars.iter().enumerate().map(|(i, f)| format!("{}. {}", i+1, f.planar)).collect::<Vec<String>>().join("\n"), false)

            .field("Light Cones", light_cones.iter().enumerate().map(|(i, (s, c))| format!("{}. {} (S{})", i+1, c.name, s)).collect::<Vec<String>>().join("\n"), false)


            // STATS
            .field("Body", chosen_build.body.iter().map(|f| f.stat.to_string()).collect::<Vec<String>>().join(" / "), true)
            .field("Feet", chosen_build.feet.iter().map(|f| f.stat.to_string()).collect::<Vec<String>>().join(" / "), true)
            .field("\u{200B}", "\u{200B}", true)
            .field("Planar Sphere", chosen_build.sphere.iter().map(|f| f.stat.to_string()).collect::<Vec<String>>().join(" / "), true)
            .field("Link Rope", chosen_build.rope.iter().map(|f| f.stat.to_string()).collect::<Vec<String>>().join(" / "), true)
            .field("\u{200B}", "\u{200B}", true)

            .field("Substats", &chosen_build.substats, false)

            // TRACES
            .field("Skill Priority", &chosen_build.skill_priority, false)
            .field("Major Traces Priority", &chosen_build.traces_priority, false)

            .to_owned()
    )
}

async fn get_character_team(character: &Character) -> Option<CreateEmbed> {
    struct Team {
        name: String,
        member_1: Character,
        member_2: Character,
        member_3: Character,
        member_4: Character
    }

    let teams = join_all(character.teams.as_ref().expect("No teams").iter().map(|team| async move {
        Team {
            name: team.name.clone(),
            member_1: get_character_data(team.member_1.clone()).await.unwrap_or_else(|| panic!("{} doesn't exist", team.member_1)),
            member_2: get_character_data(team.member_2.clone()).await.unwrap_or_else(|| panic!("{} doesn't exist", team.member_2)),
            member_3: get_character_data(team.member_3.clone()).await.unwrap_or_else(|| panic!("{} doesn't exist", team.member_3)),
            member_4: get_character_data(team.member_4.clone()).await.unwrap_or_else(|| panic!("{} doesn't exist", team.member_4))
        }
    })).await;

    Some(
        get_base_character_embed(character)
            .fields(teams.into_iter().map(|team| {
                (team.name, format!("{} / {} / {} / {}", team.member_1.name, team.member_2.name, team.member_3.name, team.member_4.name), false)
            }))
            .to_owned()
    )
}

fn create_character_tabs_button<'a>(f: &'a mut CreateComponents, char: &Character, current_tab: &CharacterTab) -> &'a mut CreateComponents {
    let mut all_buttons: Vec<CreateButton> = vec![];
    let mut home_button: CreateButton = CreateButton::default();
    let mut review_button: CreateButton = CreateButton::default();
    let mut gear_button: CreateButton = CreateButton::default();
    let mut team_button: CreateButton = CreateButton::default();

    // Home Button
    {
        match current_tab {
            CharacterTab::Home => { home_button.style(ButtonStyle::Success); }
            _ => { home_button.style(ButtonStyle::Primary); }
        };
        home_button.label("Home");
        home_button.custom_id(CharacterTab::Home.to_button_id());
        all_buttons.push(home_button);
    }


    // Review Button
    {
        match current_tab {
            CharacterTab::Review => { review_button.style(ButtonStyle::Success); }
            _ => { review_button.style(ButtonStyle::Primary); }
        };
        match &char.review {
            None => { review_button.disabled(true); }
            Some(_) => { review_button.disabled(false); }
        };
        review_button.label("Review");
        review_button.custom_id(CharacterTab::Review.to_button_id());
        all_buttons.push(review_button);
    }

    // Gear Button
    {
        match current_tab {
            CharacterTab::Gear(_) => { gear_button.style(ButtonStyle::Success); }
            _ => { gear_button.style(ButtonStyle::Primary); }
        };

        match &char.build_data {
            None => {
                gear_button.label("Gear");
                gear_button.disabled(true);
            }
            Some(_) => {
                gear_button.disabled(false);
                match current_tab {
                    CharacterTab::Gear(n) => {
                        gear_button.label(format!("Gear {}/{}", n+1, char.build_data.as_ref().expect("Cannot find gear").len()));
                    }
                    _ => {gear_button.label("Gear");}
                }
            }
        }

        gear_button.custom_id(CharacterTab::Gear(1).to_button_id());
        all_buttons.push(gear_button);
    }


    // Team Button
    {
        match current_tab {
            CharacterTab::Team => { team_button.style(ButtonStyle::Success); }
            _ => { team_button.style(ButtonStyle::Primary); }
        };
        match &char.teams {
            None => { team_button.disabled(true); }
            Some(_) => { team_button.disabled(false); }
        };
        team_button.label("Teams");
        team_button.custom_id(CharacterTab::Team.to_button_id());
        all_buttons.push(team_button);
    }


    f.create_action_row(|r| {
        all_buttons.into_iter().for_each(|b| {
            r.add_button(b);
        });
        r
    })
}


async fn menu_handler(ctx: Context<'_>, interaction: Arc<MessageComponentInteraction>, character: Character, mut tab: CharacterTab) {
    let mut looping = false;
    loop {
        let character_embed = match tab {
            CharacterTab::Home => {
                get_character_home(&character).await
            }
            CharacterTab::Review => {
                get_character_review(&character).await
            }
            CharacterTab::Gear(n) => {
                let builds = character.build_data.as_ref().expect("No builds");
                let n : usize = n % builds.len();
                tab = CharacterTab::Gear(n);
                get_character_build(&character, n).await
            }
            CharacterTab::Team => {
                get_character_team(&character).await
            }
        };
        match looping {
            true => {
                    interaction.edit_original_interaction_response(&ctx, |d| {
                        match character_embed {
                            None => {}
                            Some(_) => { d.set_embed(character_embed.unwrap()); }
                        }
                        d.content("");
                        d.components(|f| create_character_tabs_button(&mut *f, &character, &tab))
                    }).await.expect("Cannot edit interaction message");
                }

            false => {
                if (interaction
                    .create_interaction_response(&ctx, |r| {
                        r.interaction_response_data(|d| {
                            match character_embed {
                                None => {}
                                Some(_) => { d.set_embed(character_embed.unwrap()); }
                            }
                            d.content("");
                            d.ephemeral(true);
                            d.components(|f| create_character_tabs_button(&mut *f, &character, &tab))
                        })
                    }).await).is_ok() { interaction.delete_followup_message(&ctx, interaction.message.id).await.unwrap(); }
            }
        }

        let x = match interaction.get_interaction_response(&ctx).await {
            Ok(x) => { x }
            Err(_) => return
        };

        let x = x.await_component_interaction(ctx).timeout(Duration::from_secs(10 * 60)).await;

        match x {
            None => {}
            Some(x) => {
                looping = true;
                tab = match x.data.custom_id.as_str() {
                    "charactertab_home" => CharacterTab::Home,
                    "charactertab_review" => CharacterTab::Review,
                    "charactertab_gear" => {
                        match tab {
                            CharacterTab::Gear(n) => CharacterTab::Gear(n+1),
                            _ => CharacterTab::Gear(0)
                        }
                    },
                    "charactertab_team" => CharacterTab::Team,
                    _ => CharacterTab::Home
                };
                x.defer(ctx).await.expect("TODO: panic message");
            }
        }
    }
}

async fn choice_interaction_handler(ctx: Context<'_>, message: &ReplyHandle<'_>) {
    let message = message.clone().into_message().await.unwrap();
    let interaction =
        match message.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await {
            Some(x) => {
                x
            }
            None => {
                message.reply(&ctx, "Timed out").await.unwrap();
                return;
            }
        };
    let character_string = &interaction.data.values[0];
    ctx.data().registries.character_count.get_or_create(&CharacterLabels{
        character_name: character_string.clone()
    }).inc();
    let character = get_character_data(character_string.to_string()).await.unwrap();
    menu_handler(ctx, interaction, character, CharacterTab::Home).await;
}

impl CharacterTab {
    fn to_button_id(&self) -> &'static str {
        match self {
            CharacterTab::Home => "charactertab_home",
            CharacterTab::Review => "charactertab_review",
            CharacterTab::Gear(_) => "charactertab_gear",
            CharacterTab::Team => "charactertab_team"
        }
    }
}

/// Search a character info
#[poise::command(slash_command, prefix_command)]
pub async fn character(
    ctx: Context<'_>,
    #[description = "Character to Search"] user: String,
) -> Result<(), Error> {
    match get_nearest_characters(user).await {
        None => { ctx.say("Error occurred, please see logs".to_string()).await? }
        Some(characters) => {
            let handler = create_menu(ctx, characters).await;
            choice_interaction_handler(ctx, &handler).await;
            handler
        }
    };
    Ok(())
}