use crate::guard::Guard;
use crate::settings::Settings;
use actix::{Actor, ActorContext, Running, StreamHandler};
use actix_web_actors::ws;

pub mod timer;

pub struct WebData {
    pub settings: Settings,
    pub guard: Guard,
}
