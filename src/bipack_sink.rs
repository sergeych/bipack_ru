use std::iter::Iterator;

pub trait BipackSink {
    fn put_u8(self: &mut Self, data: u8) -> &Self;

    fn put_fixed_bytes(self: &mut Self, data: &[u8]) -> &Self {
        for b in data { self.put_u8(*b); }
        return self
    }

    fn put_u16(self: &mut Self, mut value: u16) -> &Self {
        let mut result = [0u8; 2];
        for i in (0..result.len()).rev() {
            result[i] = value as u8;
            println!(":: {} / {}", value, value as u8);
            value = value >> 8;
        }
        self.put_fixed_bytes(&result)
    }

    fn put_u32(self: &mut Self, mut value: u32) -> &Self {
        let mut result = [0u8; 4];
        for i in (0..result.len()).rev() {
            result[i] = value as u8;
            println!(":: {} / {}", value, value as u8);
            value = value >> 8;
        }
        self.put_fixed_bytes(&result)
    }
    fn put_u64(self: &mut Self, mut value: u64) -> &Self {
        let mut result = [0u8; 8];
        for i in (0..result.len()).rev() {
            result[i] = value as u8;
            println!(":: {} / {}", value, value as u8);
            value = value >> 8;
        }
        self.put_fixed_bytes(&result)
    }
}

const V0LIMIT: u64 = 1u64 << 6;
const V1LIMIT: u64 = 1u64 << 14;
const V2LIMIT: u64 = 1u64 << 22;

impl BipackSink for Vec<u8> {
    fn put_u8(self: &mut Self, data: u8) -> &Self {
        self.push(data);
        self
    }
}

