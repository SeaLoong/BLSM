use crate::packet::structs::VarInt;

pub mod id {
    use crate::packet::structs::VarInt;

    pub const SHOW_IDENTITY: VarInt = 0x01;
    pub const RATE_LIMIT: VarInt = 0x02;
    pub const TASK_APPLICATION: VarInt = 0x03;
    pub const TASK_CHANGE: VarInt = 0x04;
    pub const TASK_CONFIRM: VarInt = 0x05;
    pub const DATA_REPORT: VarInt = 0x06;
    pub const NOTIFICATION: VarInt = 0xFF;
}

pub mod show_identity {
    pub mod category {
        use crate::packet::structs::VarInt;

        pub const CLIENT: VarInt = 1;
        pub const SERVER: VarInt = 2;
        pub const ADMIN: VarInt = 3;
    }
}

pub mod data_report {
    pub mod category {
        use crate::packet::structs::VarInt;

        pub const STORM: VarInt = 1;
        pub const SPECIAL_GIFT: VarInt = 2;
        pub const LOTTERY: VarInt = 3;
    }
}
