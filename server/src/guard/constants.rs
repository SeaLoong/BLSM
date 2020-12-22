use crate::packet::structs::VarInt;

pub mod reason {
    pub mod kick {
        use actix_web_actors::ws::{CloseCode, CloseReason};

        const HEARTBEAT_TIMEOUT: CloseReason = CloseReason {
            code: CloseCode::from(4001),
            description: Some(String::from("heartbeat timeout")),
        };
        const RESPONSE_TIMEOUT: CloseReason = CloseReason {
            code: CloseCode::from(4002),
            description: Some(String::from("response timeout")),
        };
        const RATE_LIMIT: CloseReason = CloseReason {
            code: CloseCode::from(4003),
            description: Some(String::from("rate limit")),
        };
        const TOO_MANY_CONNECTIONS: CloseReason = CloseReason {
            code: CloseCode::from(4004),
            description: Some(String::from("too many connections")),
        };
        const UNACCEPTABLE_PACKET: CloseReason = CloseReason {
            code: CloseCode::from(4005),
            description: Some(String::from("unacceptable packet")),
        };
        const INVALID_PACKET: CloseReason = CloseReason {
            code: CloseCode::from(4006),
            description: Some(String::from("invalid packet")),
        };
        const INCORRECT_DATA_FORMAT: CloseReason = CloseReason {
            code: CloseCode::from(4007),
            description: Some(String::from("incorrect data format")),
        };
    }
    pub mod ban {
        use actix_web_actors::ws::{CloseCode, CloseReason};

        const BANNED: CloseReason = CloseReason {
            code: CloseCode::Policy,
            description: Some(String::from("banned")),
        };
        const TOO_MANY_KICKS: CloseReason = CloseReason {
            code: CloseCode::Policy,
            description: Some(String::from("too many kicks")),
        };
        const INVALID_TOKEN: CloseReason = CloseReason {
            code: CloseCode::Policy,
            description: Some(String::from("invalid token")),
        };
    }
}
