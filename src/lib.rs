//! # Bipack codec
//!
//! The set of tools to effectively encode and decode bipack values. It is internationally
//! minimalistic to be used wit Divan smart-contracts where number of instructions could
//! be important.
//!
//! - [bipack_source::BipackSource] is used to decode values, there is implementation
//!   [bipack_source::SliceSource] that parses binary slice. The trait only needs byte-read
//!   method for the implementation.
//!
//! - [bipack_sink::BipackSink] trait that is also implemented for [`Vec<u8>`] allows to encode values
//!   into the bipack format. It is the same simple to implement it for any else binary data
//!   source.
//!
//! ## Utilities
//!
//! - to siplify encoding of unsigned ints the [bipack_sink::IntoU64] trait is used with
//!   imlementation for usual u* types.
//!
//! - [tools::to_dump] utility function converts binary data into human-readable dump as in old goot
//!   times (address, bytes, ASCII characters).
//!
//! ## About Bipack format
//!
//! This is a binary format created wround the idea of bit-effectiveness and not disclosing
//! inner data structure. Unlike many known binary and text formats, liek JSON, BSON, BOSS, and
//! many others, it does not includes field names into packed binaries.
//!
//! It also uses ratinally packed variable length format very effective for unsigned integers of
//! various sizes. This implementation supports sizes for u8, u16, u32 and u64. It is capable of
//! holding longer values too but for big numbers the fixed size encoding is mostly more effective.
//! This rarional encoding format is called `smartint` and is internally used everywhere when one
//! need to pack unsigned number, unless the fixed size is important.
//! 
//! ### Varint encoding
//! 
//! Smart variable-length long encoding tools, async. It gives byte-size gain from 64 bits numbers,
//! so it is very useful when encoding big numbers or at least very bui long values. In other cases
//! [bipack_sink::BipackSink::put_unsigned] works faster, and extra bits it uses does not play
//!
//! | Bytes sz | varint bits | smartint bits |
//! |:-----:|:------:|:---------:|
//! |   1   |    7   |     6     |
//! |   2   |    14  |    14     |
//! |   3   |    21  |    22     |
//! |   4   |    28  |    29     |
//! |   5   |    35  |    36     |
//! |   6+  |    7*N |   7*N+1   |
//! |   9   |    63  |   64      |
//! |   10  |    64  |   ---     |
//!
//! In other words, except for very small numbers smartint
//! gives 1 data bit gain for the same packed byte size. For example,
//! full size 64 bits number with smartint takes one byte less (9 bytes vs. 10 in Varint).
//!
//! So, except for values in range 32..63 it gives same or better byte size effectiveness
//! than `Varint`. In particular:
//!
//! The effect of it could be interpreted as:
//!
//! | number values | size  |
//! |:--------------|:------:|
//! | 0..31 | same |
//! | 32..63 | worse 1 byte |
//! | 64..1048573 | same |
//! | 1048576..2097151 | 1 byte better |
//! | 2097152..134217727 | same |
//! | 134217728..268435456 | 1 byte better |
//!
//! etc.
//!
//! ## Encoding format
//!
//! Enncoded data could be 1 or more bytes in length. Data are
//! packed as follows:
//!
//! | byte offset | bits range | field |
//! |-------------|------------|-------|
//! | 0 | 0..1 | type |
//! | 0 | 2..7 | v0 |
//! | 1 | 0..7 | v1 (when used) |
//! | 2 | 0..7 | v2 (when used) |
//!
//! Then depending on the `type` field:
//!
//! | type | encoded |
//! |------|---------|
//! | 0 | v0 is the result 0..64 (or -32..32) |
//! | 1 | v0 ## v1 are the result, 14 bits |
//! | 2 | v0  ## v1 ## v2 are the result, 22bits
//! | 3 | v0, ## v1 ## v2 ## (varint encoded rest) |
//!
//! Where `##` means bits concatenation. The bits are interpreted as BIG ENDIAN,
//! for example `24573` will be encoded to `EA FF 02`
//!
//!

#![allow(dead_code)]
#![allow(unused_variables)]

pub mod bipack_source;
pub mod bipack_sink;
pub mod tools;

#[cfg(test)]
mod tests {
    use base64::Engine;
    use crate::bipack_sink::{BipackSink};
    use crate::bipack_source::{BipackSource, Result, SliceSource};
    use crate::tools::to_dump;

    #[test]
    fn fixed_unpack() -> Result<()> {
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
    fn smartint_unpack() -> Result<()> {
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