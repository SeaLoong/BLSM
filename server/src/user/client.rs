use crate::handler::Handler;
use crate::user::User;
use actix_web::web::Bytes;
use actix_web_actors::ws::WebsocketContext;

pub struct ClientHandler {}

impl Handler<User> for ClientHandler {
    fn show_identity(this: &mut User, data: &mut Bytes, ctx: &mut WebsocketContext<User>) {}

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
}
