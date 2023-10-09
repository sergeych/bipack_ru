use std::iter::Iterator;
use std::usize;

const V0LIMIT: u64 = 1u64 << 6;
const V1LIMIT: u64 = 1u64 << 14;
const V2LIMIT: u64 = 1u64 << 22;

/// Numeric value convertible to Unsigned 64 bit to be used
/// with [BipackSink#put_unsigned] compressed format. It is implemented fir usize
/// and u* types already.
pub trait IntoU64 {
    fn into_u64(self) -> u64;
}

macro_rules! into_u64 {
    ($($type:ident),*) => {
        $(impl IntoU64 for $type {
            fn into_u64(self) -> u64 {
                self as u64
            }
        })*
    };
}

into_u64!(u8, u16, u32, usize, u64);

/// Data sink to encode bipack binary format.
///
/// To implement just override [BipackSink::put_u8] and optionally [BipackSink::put_fixed_bytes].
///
/// Note that the sink is not returning errors, unlike [crate::bipack_source::BipackSource].
/// It is supposed that the sink has unlimited
/// size for the context of data it is used for. This is practical. For the case of overflow-aware
/// sink you can create one that ignores extra data when overflow is detected and report it
/// somehow, but for encoding it does not worth effort (data size could be estimated in advance).
pub trait BipackSink {
    fn put_u8(self: &mut Self, data: u8);

    fn put_fixed_bytes(self: &mut Self, data: &[u8]) {
        for b in data { self.put_u8(*b); }
    }

    fn put_var_bytes(self: &mut Self,data: &[u8]) {
        self.put_unsigned(data.len());
        self.put_fixed_bytes(data);
    }

    fn put_str(self: &mut Self,str: &str) {
        self.put_var_bytes(str.as_bytes());
    }

    fn put_u16(self: &mut Self, mut value: u16) {
        let mut result = [0u8; 2];
        for i in (0..result.len()).rev() {
            result[i] = value as u8;
            value = value >> 8;
        }
        self.put_fixed_bytes(&result);
    }

    fn put_u32(self: &mut Self, mut value: u32) {
        let mut result = [0u8; 4];
        for i in (0..result.len()).rev() {
            result[i] = value as u8;
            value = value >> 8;
        }
        self.put_fixed_bytes(&result);
    }
    fn put_u64(self: &mut Self, mut value: u64) {
        let mut result = [0u8; 8];
        for i in (0..result.len()).rev() {
            result[i] = value as u8;
            value = value >> 8;
        }
        self.put_fixed_bytes(&result);
    }

    /// Put unsigned value to compressed variable-length format, `Smartint` in the bipack
    /// terms. This format is used to store size of variable-length binaries and strings.
    /// Use [crate::bipack_source::BipackSource::get_unsigned] to unpack it.
    fn put_unsigned<T: IntoU64>(self: &mut Self, number: T) {
        let value = number.into_u64();
        let mut encode_seq = |ty: u8, bytes: &[u64]| {
            if bytes.len() == 0 { self.put_u8(0); } else {
                if bytes[0] as u64 > V0LIMIT { panic!("first byte is too big (internal error)"); }
                self.put_u8((ty & 0x03) | ((bytes[0] as u8) << 2));
                for i in 1..bytes.len() {
                    self.put_u8(bytes[i] as u8);
                }
            }
        };

        if value < V0LIMIT {
                encode_seq(0, &[value]);
        }
        else if value < V1LIMIT {
            encode_seq( 1, &[value & 0x3F, value >> 6]);
        }
        else if value < V2LIMIT {
            encode_seq(  2, &[value & 0x3f, value >> 6, value >> 14]);
        }
        else {
            encode_seq(3, &[value & 0x3f, value >> 6, value >> 14]);
            self.put_var_unsigned(value >> 22);
        }
    }

    fn put_var_unsigned(self: &mut Self, value: u64) {
        let mut rest = value;
        loop {
            let x = rest & 127;
            rest = rest >> 7;
            if rest > 0 {
                self.put_u8((x | 0x80) as u8);
            } else {
                self.put_u8(x as u8)
            }
            if rest == 0 { break; }
        }
    }
}


impl BipackSink for Vec<u8> {
    fn put_u8(self: &mut Self, data: u8) {
        self.push(data);
    }
}

