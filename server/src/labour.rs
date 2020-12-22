use crate::guard::constants::*;
use crate::labour::structs::ConnectionInfo;
use crate::labour::timer::Timer;
use crate::packet;
use crate::packet::structs::VarInt;
use crate::packet::{constants::id, PacketData};
use crate::settings::Settings;
use actix::{Actor, ActorContext, AsyncContext, Running, SpawnHandle, StreamHandler};
use actix_web::web::{Buf, Bytes};
use actix_web_actors::ws;
use actix_web_actors::ws::CloseCode;
use chrono::Local;
use dashmap::DashMap;
use governor::state::{InMemoryState, NotKeyed};
use governor::{clock, Quota, RateLimiter};
use log::info;
use std::cmp::max;
use std::collections::HashSet;
use std::lazy::SyncLazy;
use std::num::NonZeroU32;
use std::time::Duration;

pub mod handle;
pub mod structs;
pub mod timer;

static ID_HANDLE_MAP: SyncLazy<
    DashMap<VarInt, fn(&mut Labour, &mut Bytes, &mut ws::WebsocketContext<Labour>)>,
> = SyncLazy::new(|| {
    let m: DashMap<VarInt, fn(&mut Labour, &mut Bytes, &mut ws::WebsocketContext<Labour>)> =
        DashMap::new();
    m.insert(id::SHOW_IDENTITY, handle::show_identity);
    m.insert(id::RATE_LIMIT, handle::rate_limit);
    m.insert(id::TASK_APPLICATION, handle::task_application);
    m.insert(id::TASK_CHANGE, handle::task_change);
    m.insert(id::TASK_CONFIRM, handle::task_confirm);
    m.insert(id::NOTIFICATION, handle::notification);
    m
});

pub struct Labour {
    connection_info: ConnectionInfo,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock>,
    acceptance_ids: HashSet<VarInt>,
    response_ids: HashSet<VarInt>,
    response_timer: Timer<Self, ws::WebsocketContext<Self>>,
    heartbeat_timer: Timer<Self, ws::WebsocketContext<Self>>,
    category: VarInt,
    token: String,
}

impl Labour {
    pub fn new(connection_info: ConnectionInfo, settings: &Settings) -> Labour {
        let quota = Quota::with_period(Duration::from_millis(settings.rate_limit.interval as u64))
            .unwrap()
            .allow_burst(NonZeroU32::new(settings.rate_limit.max_burst as u32).unwrap());
        Labour {
            connection_info,
            rate_limiter: RateLimiter::direct(quota),
            acceptance_ids: HashSet::new(),
            response_ids: HashSet::new(),
            token: String::new(),
            response_timer: Timer::new(
                Duration::from_millis((settings.rate_limit.interval * 2) as u64),
                |labour, ctx| {
                    info!(
                        "No response packet received from Labour '{}' for {}s, kicked.",
                        labour.token,
                        labour.response_timer.duration.as_secs_f32()
                    );
                    labour.kick(ctx, "Response Timeout");
                },
            ),
            heartbeat_timer: Timer::new(
                Duration::from_millis(
                    (settings.rate_limit.interval * settings.rate_limit.max_burst) as u64,
                ),
                |labour, ctx| {
                    info!(
                        "No heartbeat packet received from Labour '{}' for {}s, kicked.",
                        labour.token,
                        labour.heartbeat_timer.duration.as_secs_f32()
                    );
                    labour.kick(ctx, "Heartbeat Timeout");
                },
            ),
            category: 0,
        }
    }

    pub fn sack(&mut self, ctx: &mut ws::WebsocketContext<Self>, reason: Option<ws::CloseReason>) {
        self.response_timer.stop(ctx);
        self.heartbeat_timer.stop(ctx);
        ctx.close(reason);
        ctx.stop();
    }

    pub fn kick(&mut self, ctx: &mut ws::WebsocketContext<Self>, s: &str) {
        self.response_timer.stop(ctx);
        self.heartbeat_timer.stop(ctx);
        let mut reason = ws::CloseReason::from(CloseCode::Policy);
        reason.description = Some(String::from(s));
        ctx.close(Some(reason));
        ctx.stop();
    }

    pub fn ban(&mut self, ctx: &mut ws::WebsocketContext<Self>, s: &str) {
        self.response_timer.stop(ctx);
        self.heartbeat_timer.stop(ctx);
        let mut reason = ws::CloseReason::from(CloseCode::Policy);
        reason.description = Some(String::from(s));
        ctx.close(Some(reason));
        ctx.stop();
    }
}

impl Actor for Labour {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.response_ids.insert(id::SHOW_IDENTITY);
        self.response_timer.start(ctx);
        self.heartbeat_timer.start(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Labour {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if self.rate_limiter.check().is_err() {
            self.kick(ctx, "Rate Limit");
            return;
        }
        match msg {
            Ok(ws::Message::Binary(mut bin)) => {
                while bin.has_remaining() {
                    if let Some(mut pkt) = packet::Packet::read_from_bytes(&mut bin) {
                        self.heartbeat_timer.start(ctx);
                        let mut resp = self.response_ids.contains(&pkt.id);
                        if resp {
                            self.response_timer.stop(ctx);
                            self.response_ids.remove(&pkt.id);
                        }
                        if resp || self.acceptance_ids.contains(&pkt.id) {
                            if let Some(val) = ID_HANDLE_MAP.get(&pkt.id) {
                                val.value()(self, &mut pkt.data, ctx);
                            } else {
                                self.kick(ctx, "No Permit");
                                return;
                            }
                        }
                    } else {
                        self.kick(ctx, "Invalid");
                        return;
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => self.sack(ctx, reason),
            _ => self.ban(ctx, "Unknown"),
        }
    }
}

#[test]
fn test() {}
