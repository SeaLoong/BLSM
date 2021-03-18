use crate::guard::reason;
use crate::labour::structs::State;
use crate::labour::Labour;
use crate::packet::structs::VarInt;
use crate::packet::{
    constants, Packet, PacketData, RateLimit, ShowIdentity, TaskApplication, TaskChange, ToPacket,
};
use actix::Actor;
use actix_web::web::{Bytes, BytesMut};
use actix_web_actors::ws;
use actix_web_actors::ws::WebsocketContext;
use log::info;
use std::collections::HashMap;
use std::lazy::SyncLazy;

pub fn show_identity(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn rate_limit(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {
    labour.kick(ctx, reason::kick::Reason::UnexpectedPacket);
}

pub fn task_application(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {
    if labour.state == State::Working
        && labour.category == Some(constants::show_identity::category::CLIENT)
    {
        if let Some(data) = TaskApplication::read_from_bytes(data) {
            info!(
                "Labour '{}' is applying task: {} rooms.",
                labour.token, data.room_count
            );

            ctx.binary(
                TaskChange {
                    room_count: 0,
                    room_ids: vec![],
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

pub fn task_change(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {
    labour.kick(ctx, reason::kick::Reason::UnexpectedPacket);
}

pub fn task_confirm(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn data_report(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}

pub fn notification(labour: &mut Labour, data: &mut Bytes, ctx: &mut WebsocketContext<Labour>) {}
