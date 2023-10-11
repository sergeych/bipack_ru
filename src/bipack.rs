use crate::bipack_sink::{BipackSink, IntoU64};
use crate::bipack_source::{BipackSource, Result};

/// The trait to unpack to be used in serializer to come. Please don't use it, it is
/// experimental.
pub trait BiPackable {
    fn bi_pack(self: &Self, sink: &mut impl BipackSink);
}

/// The trait need by [bipack()] macro and in the serializer to come, packs some
/// type into a generic sink.
pub trait BiUnpackable where Self: Sized {

    fn bi_unpack(source: &mut dyn BipackSource) -> Result<Self>;
}

/// Pack all arguments according to their type, using variable-length
/// encoding for integers and default encoding for binaries and string,
/// and return `Vec<u8>` with packed result.
///
/// It you need more fine-grained packing, use [BipackSink] directly.
#[macro_export]
macro_rules! bipack {
    ( $( $e: expr),* ) => {{
        let mut result = Vec::new();
        $(
            $e.bi_pack(&mut result);
        )*
        result
    }};
}

impl<T: IntoU64 + Copy> BiPackable for T {
    fn bi_pack(self: &Self, sink: &mut impl BipackSink) {
        sink.put_unsigned(self.into_u64())
    }
}

impl BiPackable for &str {
    fn bi_pack(self: &Self, sink: &mut impl BipackSink) {
        sink.put_str(self)
    }
}

macro_rules! declare_unpack_u {
    ($($type:ident),*) => {
        $(impl BiUnpackable for $type {
            fn bi_unpack(source: &mut dyn BipackSource) -> Result<$type> {
                Ok(source.get_unsigned()? as $type)
            }
        })*
    };
}

declare_unpack_u!(u16, u32, u64);

// impl<String> BiUnpackable<String> for String {
//     fn bi_unpack(source: &mut impl BipackSource) -> Result<Self> {
//         source.get_str()
//     }
// }

// impl dyn BiUnpackable<u32> {
//
// }

impl BiUnpackable for u8 {
    fn bi_unpack(source: &mut dyn BipackSource) -> Result<u8> {
        source.get_u8()
    }
}

impl BiUnpackable for String {
    fn bi_unpack(source: &mut dyn BipackSource) -> Result<String> {
        source.get_str()
    }
}


