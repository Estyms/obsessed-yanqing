use std::net::IpAddr;
use std::sync::Arc;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::encoding::text::encode;
use rocket::{routes, get, State, Config};
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use rocket::config::LogLevel;
use crate::metrics::characters::CharacterLabels;

struct RocketState {
    registry: Registry
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Empty {}

pub struct AllRegistries {
    pub character_count: Family::<CharacterLabels, Counter>,
    pub server_count: Family::<Empty, Gauge>
}

pub fn create_registries() -> AllRegistries {


    let character_count = Family::<CharacterLabels, Counter>::default();
    let server_count = Family::<Empty, Gauge>::default();

    AllRegistries {
        character_count,
        server_count
    }
}


#[get("/metrics")]
fn metrics(state: &State<RocketState>) -> String {
    let mut buffer = String::new();
    encode(&mut buffer, &state.registry).unwrap();
    buffer
}


pub async fn setup_server(all_registries: Arc<AllRegistries>) {
    if let Ok(enabled) = std::env::var("METRICS_ENABLED") {
        let test = match enabled.to_lowercase().as_str() {
            "false" => false,
            "true" => true,
            _ => false
        };
        if !test {println!("Metrics are not enabled, set METRICS_ENABLED to true to enable them !"); return;}


        let mut registry = Registry::default();

        registry.register(
            "character_command_count",
            "Number of character command used",
            all_registries.character_count.clone()
        );

        registry.register(
            "server_count",
            "Number of servers the bot is in",
            all_registries.server_count.clone()
        );

        let config = Config {
            address: IpAddr::V4("0.0.0.0".parse().unwrap()),
            port: std::env::var("METRICS_PORT").expect("No metrics found !").as_str().parse::<u16>().expect("METRICS_PORT is not in base 10"),
            log_level: LogLevel::Critical,
            ..Config::default()
        };

        rocket::custom(config).mount("/", routes![metrics]).manage(RocketState { registry }).launch().await.expect("Cannot launch Metrics Server");
    } else {
        println!("Metrics are not enabled, set METRICS_ENABLED to true to enable them !");
    }
}