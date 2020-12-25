use crate::guard::reason;
use crate::labour::structs::{ConnectionInfo, State};
use crate::packet::structs::VarInt;
use crate::packet::{constants::id, PacketData};
use crate::settings::{RateLimit, Settings};
use crate::util::timer::Timer;
use crate::{packet, GUARD};
use actix::{Actor, ActorContext, AsyncContext, Running, SpawnHandle, StreamHandler};
use actix_web::web::{Buf, Bytes};
use actix_web_actors::ws;
use actix_web_actors::ws::{CloseCode, CloseReason};
use chrono::Local;
use dashmap::DashMap;
use governor::state::{InMemoryState, NotKeyed};
use governor::{clock, Quota, RateLimiter};
use log::info;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::lazy::SyncLazy;
use std::num::NonZeroU32;
use std::time::Duration;

pub mod handle;
pub mod structs;

static HANDLE_MAP: SyncLazy<
    HashMap<VarInt, fn(&mut Labour, &mut Bytes, &mut ws::WebsocketContext<Labour>)>,
> = SyncLazy::new(|| {
    let mut m: HashMap<VarInt, fn(&mut Labour, &mut Bytes, &mut ws::WebsocketContext<Labour>)> =
        HashMap::new();
    m.insert(id::SHOW_IDENTITY, handle::show_identity);
    m.insert(id::RATE_LIMIT, handle::rate_limit);
    m.insert(id::TASK_APPLICATION, handle::task_application);
    m.insert(id::TASK_CHANGE, handle::task_change);
    m.insert(id::TASK_CONFIRM, handle::task_confirm);
    m.insert(id::NOTIFICATION, handle::notification);
    m
});

pub struct Labour {
    pub connection_info: ConnectionInfo,
    pub category: Option<VarInt>,
    pub token: String,
    state: State,
    rate_limit: RateLimit,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock>,
    response_ids: HashSet<VarInt>,
    response_timer: Timer<Self, ws::WebsocketContext<Self>>,
    heartbeat_timer: Timer<Self, ws::WebsocketContext<Self>>,
}

impl Labour {
    pub fn new(connection_info: ConnectionInfo, settings: &Settings) -> Labour {
        let quota = Quota::with_period(Duration::from_millis(settings.rate_limit.interval as u64))
            .unwrap()
            .allow_burst(NonZeroU32::new(settings.rate_limit.max_burst as u32).unwrap());
        Labour {
            connection_info,
            category: None,
            token: String::new(),
            state: State::Handshaking,
            rate_limit: settings.rate_limit.clone(),
            rate_limiter: RateLimiter::direct(quota),
            response_ids: HashSet::new(),
            response_timer: Timer::new(
                Duration::from_millis((settings.rate_limit.interval * 2) as u64),
                |labour, ctx| {
                    info!(
                        "No response packet received from Labour '{}' for {}s.",
                        labour.token,
                        labour.response_timer.duration.as_secs_f32()
                    );
                    labour.kick(ctx, reason::kick::Reason::ResponseTimeout);
                },
            ),
            heartbeat_timer: Timer::new(
                Duration::from_millis(
                    (settings.rate_limit.interval * settings.rate_limit.max_burst) as u64,
                ),
                |labour, ctx| {
                    info!(
                        "No heartbeat packet received from Labour '{}' for {}s.",
                        labour.token,
                        labour.heartbeat_timer.duration.as_secs_f32()
                    );
                    labour.kick(ctx, reason::kick::Reason::HeartbeatTimeout);
                },
            ),
        }
    }

    #[inline]
    pub fn stop_timer(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        self.response_timer.stop(ctx);
        self.heartbeat_timer.stop(ctx);
    }

    #[inline]
    pub fn sack(&mut self, ctx: &mut ws::WebsocketContext<Self>, reason: Option<CloseReason>) {
        self.stop_timer(ctx);
        GUARD.sack(ctx, reason);
    }

    #[inline]
    pub fn kick(&mut self, ctx: &mut ws::WebsocketContext<Self>, reason: reason::kick::Reason) {
        self.stop_timer(ctx);
        GUARD.kick(&self, ctx, reason);
    }

    #[inline]
    pub fn ban(&mut self, ctx: &mut ws::WebsocketContext<Self>, reason: reason::ban::Reason) {
        self.stop_timer(ctx);
        GUARD.ban(&self, ctx, reason);
    }
}

impl Actor for Labour {
    type Context = ws::WebsocketContext<Self>;

    #[inline]
    fn started(&mut self, ctx: &mut Self::Context) {
        self.response_ids.insert(id::SHOW_IDENTITY);
        self.response_timer.start(ctx);
        self.heartbeat_timer.start(ctx);
    }

    #[inline]
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    #[inline]
    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Labour {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if self.rate_limiter.check().is_err() {
            self.kick(ctx, reason::kick::Reason::RateLimit);
            return;
        }
        match msg {
            Ok(ws::Message::Binary(mut bin)) => {
                self.heartbeat_timer.start(ctx);
                while bin.has_remaining() {
                    if let Some(mut pkt) = packet::Packet::read_from_bytes(&mut bin) {
                        println!("{:?} !! {:?}", pkt, bin);
                        if let Some(val) = HANDLE_MAP.get(&pkt.id) {
                            if self.response_ids.contains(&pkt.id) {
                                self.response_timer.stop(ctx);
                                self.response_ids.remove(&pkt.id);
                            }
                            val(self, &mut pkt.data, ctx);
                            if ctx.state().alive() {
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    self.kick(ctx, reason::kick::Reason::InvalidPacket);
                    return;
                }
            }
            Ok(ws::Message::Close(reason)) => return,
            _ => self.kick(ctx, reason::kick::Reason::IncorrectDataFormat),
        }
    }
}

#[test]
fn test() {}
