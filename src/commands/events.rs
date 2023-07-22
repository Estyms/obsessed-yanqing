use std::cmp::Ordering;
use regex::{Regex};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serenity::builder::CreateEmbed;
use serenity::model::channel::Channel;
use crate::data::{Context, Error};


pub async fn get_main_prydwen() -> String {
    reqwest::get("https://www.prydwen.gg/star-rail/").await.expect("Cannot get Prydwen").text().await.expect("Cannot get data")
}

enum EventType {
    CharacterBanner,
    ConeBanner,
    Memory,
    Other,
}

struct BannerData {
    five_stars: String,
    four_stars: Vec<String>
}

struct Event {
    name: String,
    description: Option<(String, String)>,
    time: Option<String>,
    image: Option<String>,
    banner_data: Option<BannerData>,
    event_type: EventType
}

async fn get_events_from_document(doc: String) -> Vec<Event> {
    let document = Document::from(doc.as_str());
    let event_nodes = document.find(Class("event-tracker")).take(1).next();

    let mut events : Vec<Event> = vec![];

    if let Some(x) = event_nodes {
        for event in x.find(Class("accordion-item")) {
            let mut attributes =  event.attr("class").expect("").split(' ');
            if attributes.clone().count() != 2 {continue;}

            let id = attributes.find(|p| !p.to_owned().eq("accordion-item")).expect("Error while getting other id");

            let name = event.find(Class("event-name")).next().expect("Cannot find name").text();

            let image_regex = Regex::new(r".accordion-item\.idofevent button\{background-color:#[0-9a-f]{6};background-image:url\((/static/.*?\.jpg)\)}".replace("idofevent", id).as_str()).expect("Cannot created REGEX");
            let image = image_regex.captures(doc.as_str()).expect("Cannot scan document").get(1).map(|x| format!("https://www.prydwen.gg{}", x.as_str()));

            let description = event.find(Class("description")).next().map(|x| {
                let text = x.text();
                let desc = text.split(": ").collect::<Vec<&str>>();
                (desc.first().expect("").to_string(), desc.get(1).expect("").to_string())
            });

            let time = event.find(Class("time")).next().map(|x| x.text());

            let event_type = match &description {
                None => {
                    match event.find(Class("featured-characters")).next() {
                        None => EventType::ConeBanner,
                        Some(_) => EventType::CharacterBanner
                    }
                }
                Some(desc) => {
                    match desc.0.as_str() {
                        "Memory Turbulence" => EventType::Memory,
                        _ => EventType::Other
                    }
                }
            };


            let five_stars = event.find(Class("rarity-5").descendant(Name("picture")).descendant(Name("img"))).next().map(|x| x.attr("alt").expect("No alt on five star image").to_string());
            let four_stars = event.find(Class("rarity-4")).take(3).map(|four_stars_node| {
                four_stars_node.find(Name("picture").descendant(Name("img"))).next().map(|x| x.attr("alt").expect("No alt on four star image").to_string()).expect("")
            }).collect::<Vec<String>>();

            let banner_data = match five_stars {
                None => None,
                Some(x) => match four_stars.len() {
                    3 => Some(BannerData { five_stars: x, four_stars}),
                    _ => None
                }
            };

            events.push(Event {name, description, time, image, banner_data, event_type })
        }
    };
    events
}

pub async fn create_events_embeds() -> Vec<CreateEmbed> {
    let doc = get_main_prydwen().await;
    let mut events = get_events_from_document(doc).await;

    events.sort_by(|a, b| {
        match (&a.event_type,&b.event_type) {
            (EventType::CharacterBanner, EventType::ConeBanner) => Ordering::Less,
            (EventType::CharacterBanner, EventType::Other) => Ordering::Less,
            (EventType::CharacterBanner, EventType::Memory) => Ordering::Less,
            (EventType::ConeBanner, EventType::Other) => Ordering::Less,
            (EventType::ConeBanner, EventType::Memory) => Ordering::Less,
            (EventType::Other, EventType::Memory) => Ordering::Less,

            (EventType::Memory, EventType::Other) => Ordering::Greater,
            (EventType::Memory, EventType::ConeBanner) => Ordering::Greater,
            (EventType::Memory, EventType::CharacterBanner) => Ordering::Greater,
            (EventType::Other, EventType::CharacterBanner) => Ordering::Greater,
            (EventType::Other, EventType::ConeBanner) => Ordering::Greater,
            (EventType::ConeBanner, EventType::CharacterBanner) => Ordering::Greater,

            (_, _) => {Ordering::Equal}
        }
    });

    events.into_iter().map(|event| {
        let mut default = CreateEmbed::default();
        let mut embed = default
            .title(event.name);

        if let Some(image) = event.image {
            embed = embed.image(image);
        }

        if let Some(time) = event.time {
            embed = embed.description(format!("Time remaining : {}", time));
        }

        if let Some(description) = event.description {
            embed = embed.field(description.0, description.1, false);
        }

        if let Some(banner_infos) = event.banner_data {
            embed = embed
                .field("5 Stars", format!("- {}",banner_infos.five_stars), false)
                .field("4 Stars", format!("- {}", banner_infos.four_stars.join("\n- ")), false);
        }

        embed.to_owned()
    }).collect::<Vec<CreateEmbed>>()
}


/// Create a message that gets updated automatically
#[poise::command(slash_command, prefix_command)]
pub async fn create_event_message(
    ctx: Context<'_>,
    #[description = "Create Event Tab"] channel: Channel
) -> Result<(), Error> {
    let embeds = create_events_embeds().await;
    channel.id().send_message(ctx, |f| {
        f.set_embeds(embeds)
    }).await.unwrap();
    let channel_name = channel.id().name(ctx).await.unwrap();
    ctx.send(|f| {
        f.embed(|e|{
            e.title("Success !")
                .description(format!("The event message has been created in #{}", channel_name))
        }).ephemeral(true)
    }).await.expect("TODO: panic message");
    Ok(())
}