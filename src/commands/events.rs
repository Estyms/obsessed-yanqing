use std::cmp::Ordering;
use std::i32;
use std::str::Split;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{NaiveDateTime};
use regex::{Regex};
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Not, Predicate};
use serenity::builder::CreateEmbed;
use serenity::http::CacheHttp;
use serenity::model::channel::Channel;
use serenity::model::id::{ChannelId, MessageId};
use serenity::utils::{Color, Colour};
use crate::data::{Context, Error};
use crate::mongo::core::{add_discord_status_message, get_discord_status_message, StatusMessage};
use crate::utils;


pub async fn get_main_prydwen() -> String {
    reqwest::get("https://www.prydwen.gg/star-rail/").await.expect("Cannot get Prydwen").text().await.expect("Cannot get data")
}

#[derive(Clone, Debug)]
enum EventType {
    CharacterBanner,
    ConeBanner,
    Memory,
    Other,
}

#[derive(Clone, Debug)]
struct BannerData {
    five_stars: String,
    four_stars: Vec<String>
}

#[derive(Clone, Debug)]
struct EventTime {
    start: Option<i64>,
    end: Option<i64>
}

#[derive(Clone, Debug)]
struct Event {
    name: String,
    description: Option<(String, String)>,
    time: EventTime,
    image: Option<ImageType>,
    banner_data: Option<BannerData>,
    color: Option<Color>,
    event_type: EventType
}

#[derive(Clone, Debug)]
enum ImageType {
    Url(String),
    LocalPath(String)
}

struct Code {
    code: String,
    new: bool
}

fn get_current_events_from_document(doc: &str) -> Vec<Event> {
    let events = get_all_events(doc);

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time no longer works").as_secs() as i64;
    events.into_iter().filter(|p| match p.time.start {
        None => true,
        Some(start) => {
            start <= current_time && match p.time.end {
                None => true,
                Some(end) => current_time < end
            }
        }
    }).collect()
}

fn get_upcoming_events_from_document(doc: &str) -> Vec<Event> {
    let events = get_all_events(doc);

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time no longer works").as_secs() as i64;
    events.into_iter().filter(|p| match p.time.start {
        None => false,
        Some(start) => {
            start > current_time && match p.time.end {
                None => true,
                Some(end) => current_time < end
            }
        }
    }).collect()
}

fn get_all_events(doc: &str) -> Vec<Event> {
    let document = Document::from(doc);
    let nodes = document.find(Class("event-tracker")).collect::<Vec<Node>>();
    let mut events: Vec<Event> = vec![];

    let current_event_nodes = nodes.get(0);
    parse_events(&doc, current_event_nodes, &mut events);
    let upcoming_event_nodes = nodes.get(1);
    parse_events(&doc, upcoming_event_nodes, &mut events);
    events
}


fn parse_events(doc: &&str, event_nodes: Option<&Node>, events: &mut Vec<Event>) {
    if let Some(x) = event_nodes {
        for event in x.find(Class("accordion-item")) {
            let mut attributes = event.attr("class").expect("Cant find class attribute").split(' ');
            if attributes.clone().count() != 2 { continue; }


            let name = event.find(Class("event-name")).next().expect("Cannot find name").text();

            let (image, color) = get_image_color(doc, &mut attributes);

            let description = get_description(event);

            let time = get_time(event);

            let event_type = get_event_type(event, &description);

            let banner_data = get_banner_data(event);

            events.push(Event { name, description, time, image, banner_data, color, event_type })
        }
    };
}


fn get_date(date_string: &str) -> Option<String> {
    let date_regex = Regex::new(r"(?m)[0-9]{4}/(?:1[0-9]|0[1-9])/(?:0[1-9]|[1-2][0-9]|3[0-1]) (?:(?:[01][0-9]|2[0-3])|[0-9]):[0-5][0-9]").expect("Cannot compile time regex");
    date_regex.captures(date_string).map(|x| x.get(0).expect("No captures found").as_str().to_string())
}

fn get_time(event: Node) -> EventTime {
    event.find(Class("duration")).next().map(|x| {
        let text = x.text();
        let dates = Regex::new(r"[-—–]").expect("Can't create split time regex").split(text.as_str()).collect::<Vec<&str>>();
        let start_date_text = dates.first().expect("Cannot get start date string").to_owned();

        let start = get_date(start_date_text).map(|x| {
            NaiveDateTime::parse_from_str(x.as_str(), "%Y/%m/%d %H:%M").expect("Date").timestamp()
        });

        let mut end = None;
        if dates.len() == 2 {
            let end_date_text = dates.get(1).expect("Cannot get end date string").to_owned();
            end = get_date(end_date_text).map(|x| {
                NaiveDateTime::parse_from_str(x.as_str(), "%Y/%m/%d %H:%M").expect("Date").timestamp()
            });
        }

        EventTime {start, end}
    }).expect("No time found")
}


fn get_codes_from_document(doc: &str) -> Option<Vec<Code>> {
    let document = Document::from(doc);
    document.find(Class("codes")).next().map( |codes | {
        let mut x = codes.find(Class("code")).map(|code| {
            match code.find(Class("new")).next() {
                None => Code {
                    code: code.text(),
                    new: false
                },
                Some(_) => Code {
                    code: code.text().split(' ').collect::<Vec<&str>>().first().expect("Cannot get code").to_string(),
                    new: true
                },
            }
        }).collect::<Vec<Code>>();

        x.sort_by(|a,b| match (a.new,b.new) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal
        });

        x
    })
}


fn get_description(event: Node) -> Option<(String, String)> {
    let description = event.find(Class("description")).next().map(|x| {
        let text = x.text();
        let desc = text.split(": ").collect::<Vec<&str>>();
        (desc.first().expect("Cannot get description title").to_string(), desc.get(1).expect("Cannot get description text").to_string())
    });
    description
}

fn get_image_color(doc: &str, attributes: &mut Split<char>) -> (Option<ImageType>, Option<Colour>) {
    let id = attributes.find(|p| !p.to_owned().eq("accordion-item")).expect("Error while getting other id");
    let image_regex = Regex::new(r".accordion-item\.idofevent button\{background-color:#([0-9a-f]{6});background-image:url\((/static/.*?\.(jpg|webp)|data:image/webp;base64,[A-Za-z0-9+/]*[=]*)\)}".replace("idofevent", id).as_str()).expect("Cannot created REGEX");
    let captures = image_regex.captures(doc).expect("Cannot scan document");
    let color = captures.get(1).map(|c| {
        let num = i32::from_str_radix(c.as_str(), 16).expect("Cannot convert color to int");
        Color::from_rgb(((num >> 16) & 0xff) as u8, ((num >> 8) & 0xff) as u8, (num & 0xff) as u8)
    });

    let image = match captures.get(2) {
        Some(x) if x.as_str().starts_with("/static") => {
            Some(ImageType::Url(format!("https://www.prydwen.gg{}", x.as_str())))
        },
        Some(x) => {
            let b64 = x.as_str();
            Some(ImageType::LocalPath(utils::image_saver::write_image_from_b64(id.into(), b64.into())))
        },
        None => {
            None
        }
    };

    (image, color)
}

fn get_event_type(event: Node, description: &Option<(String, String)>) -> EventType {
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
    event_type
}

fn get_banner_data(event: Node) -> Option<BannerData> {
    let five_stars = event.find(Class("rarity-5").and(Class("avatar").or(Class("hsr-set-image"))).descendant(Name("img").and(Not(Attr("role", "presentation"))))).next().map(|x|{
        x.attr("alt").expect("Cannot get five star")
    }).or(None);
    let four_stars = event.find(Class("rarity-4").and(Class("avatar").or(Class("hsr-set-image")))).take(3).map(|four_stars_node| {
        four_stars_node.find(Name("img").and(Not(Attr("role", "presentation")))).next().map(|x| x.attr("alt").expect("Cannot get four stars").to_string()).expect("No four stars")
    }).collect::<Vec<String>>();

    match five_stars {
        None => None,
        Some(x) => match four_stars.len() {
            3 => Some(BannerData { five_stars: x.to_string(), four_stars }),
            _ => None
        }
    }
}

fn create_current_events_embeds(doc : &str) -> Vec<CreateEmbed> {
    let mut events = get_current_events_from_document(doc).into_iter().take(8).collect::<Vec<Event>>();

    events.sort_by(sort_events());

    events.into_iter().map(|event| {
        let mut default = CreateEmbed::default();
        let mut embed = default
            .title(event.name);

        if let Some(image) = event.image {
            match image {
                ImageType::Url(x) => {embed = embed.image(x)}
                ImageType::LocalPath(x) => {embed = embed.attachment(x)}
            }
        }

        if let Some(color) = event.color {
            embed = embed.color(color);
        }

        if let Some(time) = event.time.end {
            embed = embed.description(format!("Ends <t:{}:R>", time));
        }

        if let Some(description) = event.description {
            embed = embed.field(description.0, description.1, false);
        }

        if let Some(banner_infos) = event.banner_data {
            embed = embed
                .field("5 Stars", format!("- {}",banner_infos.five_stars), false)
                .field("4 Stars", format!("- {}", banner_infos.four_stars.join("\n- ")), false);
        }


        embed.footer(|f| f.text("Data from https://www.prydwen.gg")).to_owned()
    }).collect::<Vec<CreateEmbed>>()
}

fn create_upcoming_embed(doc : &str) -> Option<CreateEmbed> {
    let mut events = get_upcoming_events_from_document(doc);
    events.sort_by(sort_events());
    if events.is_empty() {return None;}

    let banner_events : Vec<Event> = events.iter().cloned().filter(|p| matches!(p.event_type, EventType::ConeBanner | EventType::CharacterBanner)).collect();
    let other_events: Vec<Event> = events.iter().cloned().filter(|p| !matches!(p.event_type, EventType::ConeBanner | EventType::CharacterBanner)).rev().collect();

    Some(CreateEmbed::default()
        .title("Upcoming Events")
        .field("Banners", banner_events.iter().map(|banner| {
            format!("{} - **{}** : Starts <t:{}:R>", banner.name, banner.banner_data.as_ref().expect("No Banner Data on Banner").five_stars, banner.time.start.expect("No start time for Upcomming Event"))
        }).collect::<Vec<String>>().join("\n"),false)
        .field("Events", other_events.iter().map(|event| {
            format!("{} : Starts <t:{}:R>", event.name, event.time.start.expect("No start time for Upcomming Event"))
        }).collect::<Vec<String>>().join("\n"),false)
        .color(Color::from_rgb(rand::random(), rand::random(), rand::random()))
        .footer(|f| f.text("Data from https://www.prydwen.gg"))
        .to_owned()
    )
}

fn create_codes_embed(doc : &str) -> Option<CreateEmbed> {
    let codes = get_codes_from_document(doc);
    codes.map(|codes| CreateEmbed::default()
                .title("Codes")
                .description(codes.iter().map(|code| {
                    match code.new {
                        true => format!("- {} :sparkles:", code.code),
                        false => format!("- {}", code.code)
                    }
                }).collect::<Vec<String>>().join("\n"))
        .url("https://hsr.hoyoverse.com/gift")
        .color(Color::from_rgb(rand::random(), rand::random(), rand::random()))
        .footer(|f| f.text("Data from https://www.prydwen.gg"))
        .to_owned())
}


pub async fn create_event_embeds() -> Vec<CreateEmbed> {
    let doc = get_main_prydwen().await;

    let mut embeds: Vec<CreateEmbed> = vec![];
    embeds.append(create_current_events_embeds(&doc).as_mut());
    if let Some(embed) = create_upcoming_embed(&doc) {
        embeds.push(embed);
    };
    if let Some(embed) = create_codes_embed(&doc) {
        embeds.push(embed);
    };
    embeds
}

fn sort_events() -> fn(&Event, &Event) -> Ordering {
    |a, b| {
        match (&a.event_type, &b.event_type) {
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

            (_, _) => {
                match (a.time.end, b.time.end) {
                    (Some(a_end), Some(b_end)) => {
                        match a_end > b_end {
                            true => Ordering::Greater,
                            false => Ordering::Less
                        }
                    }
                    _ => Ordering::Equal
                }
            }
        }
    }
}


/// Create a message that gets updated automatically
#[poise::command(slash_command, prefix_command)]
pub async fn create_event_message(
    ctx: Context<'_>,
    #[description = "Create Event Tab"] channel: Channel
) -> Result<(), Error> {

    let message = get_discord_status_message(ctx.guild().expect("Message not sent in guild").id.0).await;
    if let Some(e) = message {
        let rm = ChannelId::from(e.channel_id as u64)
            .message(&ctx.http(), MessageId::from(e.message_id as u64))
            .await;
        match rm {
            msg if rm.is_ok() => {
                msg.unwrap().delete(&ctx.http()).await.unwrap();
            }
            _ => {}
        }
    }

    let embeds = create_event_embeds().await;
    let files = get_files_from_embeds(&embeds);

    let msg = channel.id()
        .send_files(ctx, files.iter().map(|x| x.as_str()).collect::<Vec<&str>>(), |f| {
        f.add_embeds(embeds)
        }
     ).await.unwrap();

    let sm = StatusMessage {
        message_id: *msg.id.as_u64() as i64,
        channel_id: msg.channel_id.0 as i64,
        guild_id: ctx.guild_id().unwrap().0 as i64
    };

    add_discord_status_message(sm).await;

    let channel_name = channel.id().name(ctx).await.unwrap();

    ctx.send(|f| {
        f.embed(|e|{
            e.title("Success !")
                .description(format!("The event message has been created in #{}", channel_name))
        }).ephemeral(true)
    }).await.unwrap();

    Ok(())
}

pub fn get_files_from_embeds(embeds: &Vec<CreateEmbed>) -> Vec<String> {
    embeds.clone().into_iter().map(|e| {
        if let Some(x) = e.clone().0.get("image") {
            let name = x.as_object().unwrap().get("url").unwrap().to_string();
            let name = name.replace("\"", "");
            if name.starts_with("a") {
                Some(name.replace("attachment://", "./images/"))
            } else {
                None
            }
        } else {
            None
        }
    }).filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<String>>()
}