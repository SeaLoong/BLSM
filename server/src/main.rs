#![feature(async_closure)]
#![feature(once_cell)]
#![allow(unused)]

use crate::labour::structs::ConnectionInfo;
use crate::settings::Settings;
use crate::util::DisconnectActor;
use actix_web::{get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use log::{info, warn};
use std::io::stdin;
use std::net::SocketAddr;
use std::sync::Arc;

mod guard;
mod labour;
mod logger;
mod packet;
mod settings;
mod state;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = get_matches();
    let (mut settings, cfg) = settings::Settings::new(&matches).expect("Can't read config file!");
    logger::init_logger(&settings);
    settings.done(matches, cfg);
    let settings = web::Data::new(settings);

    info!("Bilibili Live Synergetic Monitor starts to run...");
    let addr = SocketAddr::new(settings.ip, settings.port);

    let server = HttpServer::new(move || App::new().app_data(settings.clone()).service(ws_index))
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
                "help" => {
                    println!("stop: Close all connections and stop server.");
                }
                _ => println!("Unknown command \"{}\", type \"help\" for help.", s),
            }
        }
    }
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    use clap::*;
    let app = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg debug: --debug "Enable debug mode.")
        (@arg config: -c --config +takes_value "(Optional) Path to config file.")
        (@arg ip: -i --ip +takes_value "(Optional) IP address to listen. Default value: 0.0.0.0")
        (@arg port: -u --username +takes_value "(Optional) Port to listen. Default value: 8181")
        (@arg interval: --interval +takes_value "(Optional) Minimum interval (ms) to receive request. Default value: 10000")
    );
    app.get_matches()
}

#[get("/")]
async fn ws_index(
    settings: web::Data<Settings>,
    req: HttpRequest,
    payload: web::Payload,
) -> impl Responder {
    if let Some(addr) = req.peer_addr() {
        info!("Connection incoming: {}", addr);
        ws::start(
            labour::Labour::new(ConnectionInfo::new(&req.connection_info(), addr), &settings),
            &req,
            payload,
        )
    } else {
        warn!("Unexpected!!! No SocketAddr request!!!");
        ws::start(DisconnectActor, &req, payload)
    }
}
