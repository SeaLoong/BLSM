use crate::handler::Handler;
use crate::user::User;
use actix_web::web::Bytes;
use actix_web_actors::ws::WebsocketContext;

pub struct UnknownHandler {}

impl Handler<User> for UnknownHandler {
    fn show_identity(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
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

    fn rate_limit(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
        unimplemented!()
    }

    fn task_application(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
        unimplemented!()
    }

    fn task_change(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
        unimplemented!()
    }

    fn task_confirm(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
        unimplemented!()
    }

    fn data_report(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
        unimplemented!()
    }

    fn notification(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {
        unimplemented!()
    }