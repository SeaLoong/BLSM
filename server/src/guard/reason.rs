pub mod kick {
    use actix_web_actors::ws::{CloseCode, CloseReason};
    use dashmap::DashMap;
    use std::lazy::SyncLazy;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub enum Reason {
        HeartbeatTimeout,
        ResponseTimeout,
        RateLimit,
        TooManyConnections,
        UnexpectedPacket,
        InvalidPacket,
        IncorrectDataFormat,
    }

    pub static CODE_MAP: SyncLazy<DashMap<Reason, CloseReason>> = SyncLazy::new(|| {
        let mut m: DashMap<Reason, CloseReason> = DashMap::new();
        m.insert(
            Reason::HeartbeatTimeout,
            CloseReason {
                code: CloseCode::from(4001),
                description: Some("heartbeat timeout".to_owned()),
            },
        );
        m.insert(
            Reason::ResponseTimeout,
            CloseReason {
                code: CloseCode::from(4002),
                description: Some("response timeout".to_owned()),
            },
        );
        m.insert(
            Reason::RateLimit,
            CloseReason {
                code: CloseCode::from(4003),
                description: Some("rate limit".to_owned()),
            },
        );
        m.insert(
            Reason::TooManyConnections,
            CloseReason {
                code: CloseCode::from(4004),
                description: Some(("too many connections".to_owned())),
            },
        );
        m.insert(
            Reason::UnexpectedPacket,
            CloseReason {
                code: CloseCode::from(4005),
                description: Some(("unexpected packet".to_owned())),
            },
        );
        m.insert(
            Reason::InvalidPacket,
            CloseReason {
                code: CloseCode::from(4006),
                description: Some(("invalid packet".to_owned())),
            },
        );
        m.insert(
            Reason::IncorrectDataFormat,
            CloseReason {
                code: CloseCode::from(4007),
                description: Some(("incorrect data format".to_owned())),
            },
        );
        m
    });
}
pub mod ban {
    use actix_web_actors::ws::{CloseCode, CloseReason};
    use dashmap::DashMap;
    use std::lazy::SyncLazy;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub enum Reason {
        Banned,
        TooManyKicks,
        InvalidToken,
    }

    pub static CODE_MAP: SyncLazy<DashMap<Reason, CloseReason>> = SyncLazy::new(|| {
        let mut m: DashMap<Reason, CloseReason> = DashMap::new();
        m.insert(
            Reason::Banned,
            CloseReason {
                code: CloseCode::Policy,
                description: Some("banned".to_owned()),
            },
        );
        m.insert(
            Reason::TooManyKicks,
            CloseReason {
                code: CloseCode::Policy,
                description: Some("too many kicks".to_owned()),
            },
        );
        m.insert(
            Reason::InvalidToken,
            CloseReason {
                code: CloseCode::Policy,
                description: Some("invalid token".to_owned()),
            },
        );
        m
    });
}
