/// Data source compatible with mp_bintools serialization. It supports
/// fixed-size integers in rihgt order and varint ans smartint encodings
/// separately.
pub trait BipackSource {
    fn u8(self: &mut Self) -> u8;

    fn u16(self: &mut Self) -> u16 {
        ((self.u8() as u16) << 8) + (self.u8() as u16)
    }
    fn u32(self: &mut Self) -> u32 {
        ((self.u16() as u32) << 16) + (self.u16() as u32)
    }

    fn u64(self: &mut Self) -> u64 {
        ((self.u32() as u64) << 32) | (self.u32() as u64)
    }

    fn smart_u64(self: &mut Self) -> u64 {
        let mut get = || -> u64 { self.u8() as u64 };
        let first = get();
        let mut ty = first & 3;


        let mut result = first >> 2;
        if ty == 0 { return result; }
        ty -= 1;

        result = result + (get() << 6);
        if ty == 0 { return result; }
        ty -= 1;

        result = result + (get() << 14);
        if ty == 0 { return result; }

        result | (self.var_u64() << 22)
    }

    fn var_u64(self: &mut Self) -> u64 {
        let mut result = 0u64;
        let mut count = 0;
        loop {
            let x = self.u8() as u64;
            result = result | ((x & 0x7F) << count);
            if (x & 0x80) == 0 { return result; }
            count += 7
        }
    }

    fn smart_u16(self: &mut Self) -> u16 {
        self.smart_u64() as u16
    }
    fn smart_u32(self: &mut Self) -> u32 { self.smart_u64() as u32 }
}

pub struct SliceSource<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> SliceSource<'a> {
    pub fn new(src: &'a [u8]) -> SliceSource {
        SliceSource { data: src, position: 0 }
    }
}

impl<'x> BipackSource for SliceSource<'x> {
    fn u8(self: &mut Self) -> u8 {
        let result = self.data[self.position];
        self.position += 1;
        result
    }
}

pub fn bipack_source<'b>(v: &'b [u8]) -> SliceSource<'b> {
    SliceSource::new(v)
}

