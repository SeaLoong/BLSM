use actix::{Actor, ActorContext, Running};
use actix_web_actors::ws;

pub struct DisconnectActor;

impl Actor for DisconnectActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.close(None);
        ctx.stop();
    }
}
