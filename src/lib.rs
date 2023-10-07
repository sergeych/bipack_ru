#![allow(dead_code)]
#![allow(unused_variables)]

mod bipack_source;
mod bipack_sink;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use crate::bipack_sink::{BipackSink};
    use crate::bipack_source::{bipack_source, BipackSource, SliceSource};

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
        // println!("-- {:?}", data.iter().map(|x| format!("{:0x}", x)).collect::<Vec<_>>());
        assert_eq!("07fa00000101d0000000d8cb80a02f", hex::encode(&data).as_str());
        // println!("data = {}", to_hex(&data));
    }
}