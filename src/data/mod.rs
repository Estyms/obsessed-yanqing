use std::sync::Arc;
use crate::metrics::core::AllRegistries;

pub struct Data {
    pub registries: Arc<AllRegistries>
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod core;
pub mod allcharacters;
pub mod character;
pub mod description;
pub mod proscons;
pub mod cones;

