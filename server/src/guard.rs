use std::net::{IpAddr, SocketAddr};

use crate::config;
use crate::config::Settings;
use crate::labour::Labour;
use crate::redis::Connection;
use actix::ActorContext;
use actix_web_actors::ws::{CloseReason, WebsocketContext};
use chrono::{Local, NaiveDate, NaiveDateTime};
use dashmap::DashMap;
use log::info;
use std::collections::HashMap;

pub mod reason;

#[derive(Debug)]
pub struct Guard {
    cfg: config::guard::Guard,
    con: Connection,
}

impl Guard {
    #[inline]
    pub fn new(cfg: config::guard::Guard, con: Connection) -> Guard {
        Guard { cfg, con }
    }

    /// 检验IP地址是否被ban
    #[inline]
    pub async fn check_addr(&self, addr: &SocketAddr) -> bool {
        self.con.blacklist_get(&addr.to_string())
    }

    /// 检验Token是否被ban
    #[inline]
    pub async fn check_token(&self, token: &String) -> bool {
        self.con.blacklist_get(token)
    }

    pub fn kick(
        &self,
        labour: &Labour,
        ctx: &mut WebsocketContext<Labour>,
        reason: reason::kick::Reason,
    ) {
        if ctx.state().stopping() {
            return;
        }
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
        if ctx.state().stopping() {
            return;
        }
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
        if ctx.state().stopping() {
            return;
        }
        ctx.close(reason);
        ctx.stop();
    }
}
