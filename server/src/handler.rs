use actix::Actor;
use actix_web::web::Bytes;
use actix_web_actors::ws::WebsocketContext;

pub trait Handler<A>
where
    A: Actor<Context = WebsocketContext<A>>,
{
    fn show_identity(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);

    fn rate_limit(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);

    fn task_application(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);

    fn task_change(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);

    fn task_confirm(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);

    fn data_report(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);

    fn notification(this: &mut A, data: &mut Bytes, ctx: &mut WebsocketContext<A>);
}
