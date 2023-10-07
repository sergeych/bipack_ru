use std::iter::Iterator;

pub trait DataSink {
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

impl DataSink for Vec<u8> {
    fn put_u8(self: &mut Self, data: u8) -> &Self {
        self.push(data);
        self
    }
}

const HEX_DIGS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
];
pub fn to_hex(src: &[u8]) -> Box<String> {
    let mut result   = Vec::new();
    for i in src {
        result.push( HEX_DIGS[(i>>4) as usize]);
        result.push( HEX_DIGS[(i&15) as usize]);
    }
    Box::new(String::from_iter(result))
}

