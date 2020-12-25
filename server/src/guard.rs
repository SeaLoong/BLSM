use std::net::{IpAddr, SocketAddr};

use crate::labour::Labour;
use crate::settings;
use crate::settings::Settings;
use actix::ActorContext;
use actix_web_actors::ws::{CloseReason, WebsocketContext};
use chrono::{Local, NaiveDate, NaiveDateTime};
use dashmap::DashMap;
use log::info;
use std::collections::HashMap;

pub mod reason;

#[derive(Debug)]
pub struct Guard {
    settings: settings::Guard,
    kicked_ips: DashMap<IpAddr, i32>,
    kicked_tokens: DashMap<String, i32>,
    banned_ips: DashMap<IpAddr, i64>,
    banned_tokens: DashMap<String, i64>,
}

impl Guard {
    pub fn new(settings: &Settings) -> Guard {
        Guard {
            settings: settings.guard.clone(),
            kicked_ips: DashMap::new(),
            kicked_tokens: DashMap::new(),
            banned_ips: DashMap::new(),
            banned_tokens: DashMap::new(),
        }
    }

    pub fn check_addr(&self, addr: &SocketAddr) -> bool {
        if let Some(t) = self.banned_ips.get(&addr.ip()) {
            return t.value() < &Local::now().timestamp();
        }
        true
    }

    pub fn check_token(&self, token: &String) -> bool {
        if let Some(t) = self.banned_tokens.get(token) {
            return t.value() < &Local::now().timestamp();
        }
        true
    }

    pub fn kick(
        &self,
        labour: &Labour,
        ctx: &mut WebsocketContext<Labour>,
        reason: reason::kick::Reason,
    ) {
        let reason = reason::kick::CODE_MAP.get(&reason).unwrap().value().clone();
        let ip = &labour.connection_info.peer_addr.ip();
        let v1 = if let Some(v) = self.kicked_ips.get(ip) {
            v.value() + 1
        } else {
            1
        };
        self.kicked_ips.insert(*ip, v1);
        info!(
            "Kick ip '{}', which has {} records. Reason: {}.",
            ip,
            v1,
            reason
                .description
                .as_ref()
                .unwrap_or(&"no reason".to_owned())
        );
        let token = &labour.token;
        let v2 = if let Some(v) = self.kicked_tokens.get(token) {
            v.value() + 1
        } else {
            1
        };
        self.kicked_tokens.insert(token.clone(), v2);
        info!(
            "Kick token '{}', which has {} records. Reason: {}.",
            token,
            v2,
            reason
                .description
                .as_ref()
                .unwrap_or(&"no reason".to_owned())
        );
        if v1 % self.settings.kick_count == 0 || v2 % self.settings.kick_count == 0 {
            return self.ban(labour, ctx, reason::ban::Reason::TooManyKicks);
        }
        self.sack(ctx, Some(reason));
    }

    pub fn ban(
        &self,
        labour: &Labour,
        ctx: &mut WebsocketContext<Labour>,
        reason: reason::ban::Reason,
    ) {
        let reason = reason::ban::CODE_MAP.get(&reason).unwrap().value().clone();
        let t = Local::now().timestamp() + self.settings.ban_time * 3600;
        let ip = &labour.connection_info.peer_addr.ip();
        if !self.banned_ips.contains_key(ip) {
            self.banned_ips.insert(*ip, t);
            info!(
                "Ban ip '{}' until '{}'. Reason: {}.",
                ip,
                NaiveDateTime::from_timestamp(t, 0),
                reason
                    .description
                    .as_ref()
                    .unwrap_or(&"no reason".to_owned())
            );
        }
        let token = &labour.token;
        if !self.banned_tokens.contains_key(token) {
            self.banned_tokens.insert(token.clone(), t);
            info!(
                "Ban token '{}' until '{}'. Reason: {}.",
                ip,
                NaiveDateTime::from_timestamp(t, 0),
                reason
                    .description
                    .as_ref()
                    .unwrap_or(&"no reason".to_owned())
            );
        }
        self.sack(ctx, Some(reason));
    }

    pub fn sack(&self, ctx: &mut WebsocketContext<Labour>, reason: Option<CloseReason>) {
        ctx.close(reason);
        ctx.stop();
    }
}
