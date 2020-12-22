use crate::labour::Labour;
use crate::packet::{PacketData, ShowIdentity};
use actix::Actor;
use actix_web::web::Bytes;
use actix_web_actors::ws;

pub fn show_identity(
    labour: &mut Labour,
    data: &mut Bytes,
    ctx: &mut ws::WebsocketContext<Labour>,
) {
    if let Some(data) = ShowIdentity::read_from_bytes(data) {
        labour.category = data.category;
        labour.token = data.token;
        return;
    }
}

pub fn rate_limit(labour: &mut Labour, data: &mut Bytes, ctx: &mut ws::WebsocketContext<Labour>) {}

pub fn task_application(
    labour: &mut Labour,
    data: &mut Bytes,
    ctx: &mut ws::WebsocketContext<Labour>,
) {
}

pub fn task_change(labour: &mut Labour, data: &mut Bytes, ctx: &mut ws::WebsocketContext<Labour>) {}

pub fn task_confirm(labour: &mut Labour, data: &mut Bytes, ctx: &mut ws::WebsocketContext<Labour>) {
}

pub fn data_report(labour: &mut Labour, data: &mut Bytes, ctx: &mut ws::WebsocketContext<Labour>) {}

pub fn notification(labour: &mut Labour, data: &mut Bytes, ctx: &mut ws::WebsocketContext<Labour>) {
}
