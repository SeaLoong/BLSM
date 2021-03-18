use crate::handler::Handler;
use crate::labour::structs::ConnectionInfo;
use crate::packet::structs::VarInt;
use crate::util::timer::Timer;
use actix_web_actors::ws;
use governor::state::{InMemoryState, NotKeyed};
use governor::{clock, RateLimiter};
use std::collections::HashSet;
use std::sync::Arc;

pub mod client;
pub mod unknown;

pub struct User {
    pub connection_info: ConnectionInfo,
    pub token: String,
    pub rooms: Vec<Arc<String>>,
    handler: Box<dyn Handler<Self>>,
    guard: Arc<RwLock<Guard>>,
    state: State,
    rate_limit: RateLimit,
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock>,
    response_ids: HashSet<VarInt>,
    response_timer: Timer<Self, ws::WebsocketContext<Self>>,
    heartbeat_timer: Timer<Self, ws::WebsocketContext<Self>>,
}

impl User {
    pub fn new(
        connection_info: ConnectionInfo,
        settings: Arc<RwLock<Settings>>,
        guard: Arc<RwLock<Guard>>,
    ) -> Labour {
        let rate_limit = { settings.read().unwrap().rate_limit.clone() };
        let quota = Quota::with_period(Duration::from_millis(rate_limit.interval as u64))
            .unwrap()
            .allow_burst(NonZeroU32::new(rate_limit.max_burst as u32).unwrap());
        Labour {
            connection_info,
            category: None,
            token: String::new(),
            state: State::Handshaking,
            rate_limiter: RateLimiter::direct(quota),
            response_ids: HashSet::new(),
            response_timer: Timer::new(
                Duration::from_millis((rate_limit.interval * 2) as u64),
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
                Duration::from_millis((rate_limit.interval * rate_limit.max_burst) as u64),
                |labour, ctx| {
                    info!(
                        "No heartbeat packet received from Labour '{}' for {}s.",
                        labour.token,
                        labour.heartbeat_timer.duration.as_secs_f32()
                    );
                    labour.kick(ctx, reason::kick::Reason::HeartbeatTimeout);
                },
            ),
            settings,
            guard,
            rate_limit,
            rooms: vec![],
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
        self.guard.read().unwrap().sack(ctx, reason);
    }

    #[inline]
    pub fn kick(&mut self, ctx: &mut ws::WebsocketContext<Self>, reason: reason::kick::Reason) {
        self.stop_timer(ctx);
        self.guard.read().unwrap().kick(&self, ctx, reason);
    }

    #[inline]
    pub fn ban(&mut self, ctx: &mut ws::WebsocketContext<Self>, reason: reason::ban::Reason) {
        self.stop_timer(ctx);
        self.guard.read().unwrap().ban(&self, ctx, reason);
    }
}

impl Actor for Labour {
    type Context = ws::WebsocketContext<Self>;

    #[inline]
    fn started(&mut self, ctx: &mut Self::Context) {
        // 连接建立后要等待客户端发包，因此直接开始计时
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
        // 检查请求速率
        if self.rate_limiter.check().is_err() {
            self.kick(ctx, reason::kick::Reason::RateLimit);
            return;
        }
        match msg {
            Ok(ws::Message::Binary(mut bin)) => {
                // 重置心跳计时器
                self.heartbeat_timer.start(ctx);
                // 分离数据包
                while bin.has_remaining() {
                    if let Some(mut pkt) = packet::Packet::read_from_bytes(&mut bin) {
                        // println!("{:?}", pkt);
                        if let Some(val) = HANDLE_MAP.get(&pkt.id) {
                            if self.response_ids.contains(&pkt.id) {
                                self.response_timer.stop(ctx);
                                self.response_ids.remove(&pkt.id);
                            }
                            // 调用handle
                            val(self, &mut pkt.data, ctx);
                            // 每次handle结束后要判断上下文是否可用(比如handle中出现kick)
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
