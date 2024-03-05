# Obsessed Yanqing

This is a bot whose main purpose is to give as much information as possible about Honkai Star Rail.

> ⚠️ Disclaimer :  All Data is fetched from [Prydwen.gg](https://prydwen.gg)

## Features

### Event messages
The command `/create_event_message <channel>` creates, as its name indicate, an event message in the channel of your 
choice.</br>This message will contain:
 - All the currently running banners
 - The currently going events
 - The upcoming banners and events
 - The currently known redeemable codes

> In order to update this message every hour, three information are stored in a database : Guild id, Channel id, Message id.
> 
> ❗ Those are the only sensibles information stored by the bot !


### Character Data

By using the command `/character <character>`, you get access to loads of data about the selected character.

Such as its rarity, role, description, pros and cons, its different builds, and the recommended teams.

All the navigation is done by click and touch on the buttons bellow the message.

If a character has multiple builds/gears, you need to click on th gear button again to navigate to the next one.

## Setup

If you want to setup this bot by yourself, a docker image is available on [my docker registry](https://registry.regnault.dev/#!/taglist/obsessed-yanqing).

You'll also need a mongoDB server alongside it.

### ENV
```env
TOKEN : The discord bot's token
MONGO_HOST : The hostname of the mongodb instance
MONGO_PORT : The port of the mongodb instance
METRICS_ENABLED : Put to true to enable metrics
METRICS_PORT : Port on which the metrics server will listen
```

## Metrics

A prometheus compatible server is running in the background of the bot running on the port 8080.<br/>
For the moment, it provides two information:
- `server_count` The number of server the bot is in.
- `character_command_count` The number of time a character has been searched.

## APIs / Libraries

- [Prydwen.gg](https://www.prydwen.gg/star-rail/)
- [Serenity](https://github.com/serenity-rs/serenity) 
- [Poise](https://github.com/serenity-rs/poise)
