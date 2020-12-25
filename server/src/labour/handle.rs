use crate::guard::reason;
use crate::labour::structs::State;
use crate::labour::Labour;
use crate::packet::structs::VarInt;
use crate::packet::{Packet, PacketData, RateLimit, ShowIdentity, ToPacket};
use actix::Actor;
use actix_web::web::{Bytes, BytesMut};
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;
use log::info;
use std::collections::HashMap;
use std::lazy::SyncLazy;

pub fn show_identity(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {
    if labour.state == State::Handshaking && labour.category.is_none() {
        if let Some(data) = ShowIdentity::read_from_bytes(data) {
            labour.category = Some(data.category);
            labour.token = data.token;
            info!("Labour '{}' is employed.", labour.token);
            ctx.binary(
                RateLimit {
                    interval: labour.rate_limit.interval as VarInt,
                    max_burst: labour.rate_limit.max_burst as VarInt,
                }
                .to_packet()
                .to_bytes(),
            );
            return;
        }
        labour.kick(ctx, reason::kick::Reason::InvalidPacket);
    }
    labour.kick(ctx, reason::kick::Reason::UnexpectedPacket);
}

pub fn rate_limit(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn task_application(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {
}

pub fn task_change(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn task_confirm(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn data_report(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn notification(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}
