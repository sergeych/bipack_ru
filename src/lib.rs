//! # pipack codec
//!
//! The set of tools to effectively encode and decode bipack values. It is internationally
//! minimalistic to be used wit Divan smart-contracts where number of instructions could
//! be important.
//!
//! - [bipack_source::BipackSource] is used to decode values, there is implementation
//!   [bipack_source::SliceSource] that parses binary slice. The trait only needs byte-read
//!   method for the implementation.
//! - [bipack_sink::bipack_source]
//!
//!
#![allow(dead_code)]
#![allow(unused_variables)]

mod bipack_source;
mod bipack_sink;
mod to_dump;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use base64::Engine;
    use crate::bipack_sink::{BipackSink};
    use crate::bipack_source::{BipackSource, Res, SliceSource};
    use crate::to_dump::to_dump;

    #[test]
    fn fixed_unpack() -> Result<(),Box<dyn Error>> {
        let mut src = Vec::new();
        base64::engine::general_purpose::STANDARD_NO_PAD
            .decode_vec("B/oAAAEB0AAAANjLgKAv", &mut src)
            .expect("decoded vector");
        println!(": {}", hex::encode(&src));
        let mut ss = SliceSource::from(&src);
        let d7 = ss.get_u8()?;
        assert_eq!(7, ss.get_u8()?);
        assert_eq!(64000, ss.get_u16()?);
        assert_eq!(66000, ss.get_u32()?);
        assert_eq!(931127140399, ss.get_u64()?);
        Ok(())
    }

    #[test]
    fn smartint_unpack() -> Res<()> {
        let mut src = Vec::new();
        base64::engine::general_purpose::STANDARD_NO_PAD
            .decode_vec("BwLoA0IHBL+AAq7GDQ", &mut src)
            .expect("decoded vector");
        // println!("{}", hex::encode(&src));
        let mut ss = SliceSource::from(&src);
        assert_eq!(7, ss.get_u8()?);
        assert_eq!(64000, ss.get_packed_u16()?);
        assert_eq!(66000, ss.get_packed_u32()?);
        assert_eq!(931127140399, ss.get_unsigned()?);
        Ok(())
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
        println!("size ${}\n{}",data.len(), to_dump(&data));
        let mut src = SliceSource::from(&data);
        assert_eq!("Hello, rupack!", src.str().unwrap());
    }

    #[test]
    fn test_dump() {
        for l in 0..64 {
            let mut d2 = Vec::new();
            for u in 0..l {
                d2.push(u as u8);
            }
            println!("size {}\n{}", d2.len(), to_dump(&d2));
        }
    }
}