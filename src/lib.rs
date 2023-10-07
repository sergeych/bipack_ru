#![allow(dead_code)]
#![allow(unused_variables)]

mod bipack_source;
mod bipack_sink;
mod to_dump;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use crate::bipack_sink::{BipackSink};
    use crate::bipack_source::{bipack_source, BipackSource, SliceSource};
    use crate::to_dump::to_dump;

    #[test]
    fn fixed_unpack() {
        let mut src = Vec::new();
        base64::engine::general_purpose::STANDARD_NO_PAD
            .decode_vec("B/oAAAEB0AAAANjLgKAv", &mut src)
            .expect("decoded vector");
        println!(": {}", hex::encode(&src));
        let mut ss = SliceSource::new(&src);
        assert_eq!(7, ss.u8());
        assert_eq!(64000, ss.u16());
        assert_eq!(66000, ss.u32());
        assert_eq!(931127140399, ss.u64());
    }

    #[test]
    fn smartint_unpack() {
        let mut src = Vec::new();
        base64::engine::general_purpose::STANDARD_NO_PAD
            .decode_vec("BwLoA0IHBL+AAq7GDQ", &mut src)
            .expect("decoded vector");
        // println!("{}", hex::encode(&src));
        let mut ss = bipack_source(&src);
        assert_eq!(7, ss.u8());
        assert_eq!(64000, ss.smart_u16());
        assert_eq!(66000, ss.smart_u32());
        assert_eq!(931127140399, ss.smart_u64());
    }

    #[test]
    fn fixed_pack() {
        let mut data: Vec<u8> = Vec::new();
        data.put_u8(7);
        data.put_u16(64000);
        data.put_u32(66000);
        data.put_u64(931127140399);
        assert_eq!("07fa00000101d0000000d8cb80a02f", hex::encode(&data));
    }

    #[test]
    fn smart_pack() {
        let mut data: Vec<u8> = Vec::new();
        data.put_u8(7);
        data.put_unsigned(64000u16);
        data.put_unsigned(66000u32);
        data.put_unsigned(931127140399u64);
        // println!("?? {}", hex::encode(&data));
        assert_eq!("0702e803420704bf8002aec60d", hex::encode(&data));
    }

    #[test]
    fn pack_varbinaries_and_string() {
        let mut data = Vec::<u8>::new();
        data.put_str("Hello, rupack!");
        println!("{}",to_dump(&data));
        let mut src = bipack_source(&data);
        assert_eq!("Hello, rupack!", src.str().unwrap());
    }

    #[test]
    fn test_dump() {
        for l in 1..64 {
            let mut d2 = Vec::new();
            for u in 0..l {
                d2.push(u as u8);
            }
            println!("{}", to_dump(&d2));
        }
    }
}