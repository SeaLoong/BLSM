#![feature(once_cell)]
#![allow(unused)]

use std::io::stdin;
use std::lazy::SyncLazy;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use log::{info, warn};

use crate::config::Settings;
use crate::guard::Guard;
use crate::labour::structs::ConnectionInfo;
use crate::util::WebData;

mod config;
mod guard;
mod handler;
mod http;
mod labour;
mod logger;
mod packet;
mod pool;
mod redis;
mod state;
mod user;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cfg = config::load_and_save("config.yml");
    logger::init_logger(&cfg.log);
    config::display(&cfg);
    let addr = SocketAddr::new(cfg.ip, cfg.port);
    let mut guard = Arc::new(RwLock::new(Guard::new(&settings)));
    let mut settings = Arc::new(RwLock::new(settings));
    let data = web::Data::new(WebData {
        settings: settings.clone(),
        guard: guard.clone(),
    });
    info!("Bilibili Live Synergetic Monitor starts to run...");
    let server = HttpServer::new(move || App::new().app_data(data.clone()).service(ws_index))
        .bind(&addr)?
        .run();
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s)?;
        let s = s.trim();
        if s.len() > 0 {
            match &*s {
                "stop" => {
                    info!("Bilibili Live Synergetic Monitor is stopping.");
                    server.stop(false).await;
                    info!("Bilibili Live Synergetic Monitor has stopped.");
                    return Ok(());
                }
                "reload" => {
                    println!("Reloading config file...");
                    *settings.write().unwrap() =
                        config::Settings::new("config.yml").expect("Can't read config file!");
                    display_config(&settings.read().unwrap());
                    println!("Reloaded!");
                }
                "help" => {
                    println!("stop: Close all connections and stop server.");
                }
                _ => println!("Unknown command \"{}\", type \"help\" for help.", s),
            }
        }
    }
}

#[get("/")]
async fn ws_index(
    data: web::Data<WebData>,
    req: HttpRequest,
    payload: web::Payload,
) -> impl Responder {
    if let Some(addr) = req.peer_addr() {
        if !&data.guard.read().unwrap().check_addr(&addr) {
            return None;
        }
        info!("Connection incoming: '{}'.", addr);
        Some(ws::start(
            labour::Labour::new(
                ConnectionInfo::new(&req.connection_info(), addr),
                data.settings.clone(),
                data.guard.clone(),
            ),
            &req,
            payload,
        ))
    } else {
        // should be unreachable
        warn!("Unexpected!!! No SocketAddr request!!!");
        None
    }
}
