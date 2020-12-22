use actix_web::web::{Buf, BufMut, Bytes, BytesMut};

pub type VarInt = u32;

pub fn sizeof_varint(mut v: VarInt) -> usize {
    let mut r = 1;
    while (v >> 7) > 0 {
        r += 1;
        v >>= 7;
    }
    r
}

pub fn sizeof_string(s: &str) -> usize {
    let len = s.as_bytes().len();
    sizeof_varint(len as u32) + len
}

pub trait BufExt: Buf {
    fn get_varint(&mut self) -> Option<VarInt> {
        let mut v = 0u32;
        let mut i = 0;
        let mut b = 0x80;
        while i < 5 && (b & 0x80) != 0 {
            if self.remaining() < 1 {
                return None;
            }
            b = self.get_u8();
            v |= ((b & 0x7F) as u32) << (i * 7);
            i += 1;
        }
        Some(v)
    }
    fn get_string(&mut self) -> Option<String> {
        let len = self.get_varint()? as usize;
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            if self.remaining() < 1 {
                return None;
            }
            v.push(self.get_u8());
        }
        String::from_utf8(v).ok()
    }
}

pub trait BufMutExt: BufMut {
    fn put_varint(&mut self, mut v: VarInt) {
        while (v >> 7) > 0 {
            self.put_u8((v as u8) & 0x7F | 0x80);
            v >>= 7;
        }
        self.put_u8((v as u8) & 0x7F);
    }
    fn put_string(&mut self, s: &str) {
        self.put_varint(s.len() as VarInt);
        self.put_slice(s.as_bytes());
    }
}

impl<T: Buf> BufExt for T {}
impl<T: BufMut> BufMutExt for T {}
