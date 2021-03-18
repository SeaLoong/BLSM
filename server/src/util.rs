use crate::config::Settings;
use crate::guard::Guard;
use actix::{Actor, ActorContext, Running, StreamHandler};
use actix_web_actors::ws;
use std::sync::{Arc, RwLock};

pub mod timer;

pub struct WebData {
    pub settings: Arc<RwLock<Settings>>,
    pub guard: Arc<RwLock<Guard>>,
}
