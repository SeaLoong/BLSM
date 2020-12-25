use crate::packet::constants::*;
use crate::packet::structs::*;
use actix_web::web::{BufMut, Bytes, BytesMut};
use std::any::{Any, TypeId};

pub mod constants;
pub mod structs;

pub trait PacketData {
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self>
    where
        Self: Sized;
    fn write_to_bytes(&self, bytes: &mut BytesMut);
}

pub trait ToPacket {
    fn to_packet(&self) -> Packet;
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet {
    pub length: VarInt,
    pub id: VarInt,
    pub data: Bytes,
}

impl PacketData for Packet {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let length = bytes.get_varint()?;
        let id = bytes.get_varint()?;
        if length as usize > bytes.len() {
            return None;
        }
        let data = bytes.split_to(length as usize);
        Some(Self { length, id, data })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.length);
        bytes.put_varint(self.id);
        bytes.put(&self.data[..]);
    }
}

impl Packet {
    #[inline]
    pub fn to_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        self.write_to_bytes(&mut bytes);
        bytes.freeze()
    }
}

fn to_packet<T: PacketData>(data: &T, id: VarInt) -> Packet {
    let mut bytes = BytesMut::new();
    data.write_to_bytes(&mut bytes);
    Packet {
        length: bytes.len() as VarInt,
        id,
        data: bytes.freeze(),
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShowIdentity {
    pub category: VarInt,
    pub token: String,
}

impl PacketData for ShowIdentity {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let category = bytes.get_varint()?;
        let token = bytes.get_string()?;
        Some(Self { category, token })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.category);
        bytes.put_string(&self.token);
    }
}

impl ToPacket for ShowIdentity {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::SHOW_IDENTITY)
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RateLimit {
    pub interval: VarInt,
    pub max_burst: VarInt,
}

impl PacketData for RateLimit {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let interval = bytes.get_varint()?;
        let max_burst = bytes.get_varint()?;
        Some(Self {
            interval,
            max_burst,
        })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.interval);
        bytes.put_varint(self.max_burst);
    }
}

impl ToPacket for RateLimit {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::RATE_LIMIT)
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskApplication {
    pub room_count: VarInt,
}

impl PacketData for TaskApplication {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let room_count = bytes.get_varint()?;
        Some(Self { room_count })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.room_count);
    }
}

impl ToPacket for TaskApplication {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::TASK_APPLICATION)
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskChange {
    pub room_count: VarInt,
    pub room_ids: Vec<String>,
}

impl PacketData for TaskChange {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let room_count = bytes.get_varint()?;
        let mut room_ids = Vec::with_capacity(room_count as usize);
        for _ in 0..room_count {
            room_ids.push(bytes.get_string()?)
        }
        Some(Self {
            room_count,
            room_ids,
        })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.room_count);
        for s in &self.room_ids {
            bytes.put_string(s);
        }
    }
}

impl ToPacket for TaskChange {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::TASK_CHANGE)
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskConfirm {
    pub room_count: VarInt,
    pub room_ids: Vec<String>,
}

impl PacketData for TaskConfirm {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let room_count = bytes.get_varint()?;
        let mut room_ids = Vec::with_capacity(room_count as usize);
        for _ in 0..room_count {
            room_ids.push(bytes.get_string()?)
        }
        Some(Self {
            room_count,
            room_ids,
        })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.room_count);
        for s in &self.room_ids {
            bytes.put_string(s);
        }
    }
}

impl ToPacket for TaskConfirm {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::TASK_CONFIRM)
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataReport {
    pub category: VarInt,
    pub room_id: String,
    pub id: String,
    pub time: VarInt,
    pub detail: String,
}

impl PacketData for DataReport {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let category = bytes.get_varint()?;
        let room_id = bytes.get_string()?;
        let id = bytes.get_string()?;
        let time = bytes.get_varint()?;
        let detail = bytes.get_string()?;
        Some(Self {
            category,
            room_id,
            id,
            time,
            detail,
        })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.category);
        bytes.put_string(&self.room_id);
        bytes.put_string(&self.id);
        bytes.put_varint(self.time);
        bytes.put_string(&self.detail);
    }
}

impl ToPacket for DataReport {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::DATA_REPORT)
    }
}

/* ====================================== */

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Notification {
    pub category: VarInt,
    pub message: String,
    pub token: String,
}

impl PacketData for Notification {
    #[inline]
    fn read_from_bytes(bytes: &mut Bytes) -> Option<Self> {
        let category = bytes.get_varint()?;
        let message = bytes.get_string()?;
        let token = bytes.get_string()?;
        Some(Self {
            category,
            message,
            token,
        })
    }

    #[inline]
    fn write_to_bytes(&self, bytes: &mut BytesMut) {
        bytes.put_varint(self.category);
        bytes.put_string(&self.message);
        bytes.put_string(&self.token);
    }
}

impl ToPacket for Notification {
    #[inline]
    fn to_packet(&self) -> Packet {
        to_packet(self, id::NOTIFICATION)
    }
}

/* ====================================== */

#[test]
fn test() {
    use crate::packet::constants::*;
    let mut bytes = BytesMut::new();
    bytes.put_varint(u32::MAX >> 2);
    println!("{:?}", bytes.clone());
    assert_eq!(bytes.get_varint().unwrap(), u32::MAX >> 2);
    println!("{:?}", bytes.clone());
    println!("=======================");
    let mut bytes = BytesMut::new();
    bytes.put_string("你好");
    println!("{:?}", bytes.clone());
    assert_eq!(bytes.get_string(), Some(String::from("你好")));
    println!("{:?}", bytes.clone());
    println!("=======================");
    let mut bytes = BytesMut::new();
    let p1 = Packet {
        length: 5,
        id: id::SHOW_IDENTITY,
        data: {
            let mut b = BytesMut::new();
            ShowIdentity {
                category: show_identity::category::CLIENT,
                token: String::from("aaa"),
            }
            .write_to_bytes(&mut b);
            b.freeze()
        },
    };
    p1.write_to_bytes(&mut bytes);
    println!("{:?}", bytes.clone());
    let mut p2 = Packet::read_from_bytes(&mut bytes.clone().freeze()).unwrap();
    println!("{:?}", ShowIdentity::read_from_bytes(&mut p2.data));
    println!("{:?}", bytes.clone());
}
