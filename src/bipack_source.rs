// Copyright 2023 by Sergey S. Chernov.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;
use crate::bipack_source::BipackError::NoDataError;

/// Result of error-aware bipack function
pub type Result<T> = std::result::Result<T, BipackError>;

/// There is not enought data to fulfill the request
#[derive(Debug, Clone)]
pub enum BipackError {
    NoDataError,
    BadEncoding(FromUtf8Error),
}

impl Display for BipackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for BipackError {}


/// Data source compatible with mp_bintools serialization. It supports
/// fixed-size integers in right order and varint ans smartint encodings
/// separately. There is out of the box implementation for [`Vec<u8>`], and
/// it is easy to implements your own.
///
/// To implement source for other type, implement just [BipackSource::get_u8] or mayve also
/// [BipackSource::get_fixed_bytes] for effectiveness.
///
/// Unlike the [crate::bipack_sink::BipackSink] the source is returning errors. This is because
/// it often appears when reading data do not correspond to the packed one, and this is an often
/// case that requires proper reaction, not just a panic attack :)
pub trait BipackSource {
    fn get_u8(self: &mut Self) -> Result<u8>;

    fn get_u16(self: &mut Self) -> Result<u16> {
        Ok(((self.get_u8()? as u16) << 8) + (self.get_u8()? as u16))
    }
    fn get_u32(self: &mut Self) -> Result<u32> {
        Ok(((self.get_u16()? as u32) << 16) + (self.get_u16()? as u32))
    }

    fn get_u64(self: &mut Self) -> Result<u64> {
        Ok(((self.get_u32()? as u64) << 32) | (self.get_u32()? as u64))
    }

    /// Unpack variable-length packed unsigned value, used aslo internally to store size
    /// of arrays, binary data, strings, etc. To pack use
    /// [crate::bipack_sink::BipackSink::put_unsigned()].
    fn get_unsigned(self: &mut Self) -> Result<u64> {
        let mut get = || -> Result<u64> { Ok(self.get_u8()? as u64) };
        let first = get()?;
        let mut ty = first & 3;


        let mut result = first >> 2;
        if ty == 0 { return Ok(result); }
        ty -= 1;

        result = result + (get()? << 6);
        if ty == 0 { return Ok(result); }
        ty -= 1;

        result = result + (get()? << 14);
        if ty == 0 { return Ok(result); }

        Ok(result | (self.get_varint_unsigned()? << 22))
    }

    /// read 8-bytes varint-packed unsigned value from the source. We dont' recommend
    /// using it directly; use [BipackSource::get_unsigned] instead.
    fn get_varint_unsigned(self: &mut Self) -> Result<u64> {
        let mut result = 0u64;
        let mut count = 0;
        loop {
            let x = self.get_u8()? as u64;
            result = result | ((x & 0x7F) << count);
            if (x & 0x80) == 0 { return Ok(result); }
            count += 7
        }
    }

    /// read 2-bytes unsigned value from the source as smartint-encoded, same as
    /// [BipackSource::get_unsigned] as u16
    fn get_packed_u16(self: &mut Self) -> Result<u16> {
        Ok(self.get_unsigned()? as u16)
    }

    /// read 4-bytes unsigned value from the source
    /// read 2-bytes unsigned value from the source as smartint-encoded, same as
    /// [BipackSource::get_unsigned] as u32.
    fn get_packed_u32(self: &mut Self) -> Result<u32> { Ok(self.get_unsigned()? as u32) }

    /// read exact number of bytes from the source as a vec.
    fn get_fixed_bytes(self: &mut Self, size: usize) -> Result<Vec<u8>> {
        let mut result = Vec::with_capacity(size);
        for i in 0..size { result.push(self.get_u8()?); }
        Ok(result)
    }

    /// Read variable-length byte array from the source (with packed size), created
    /// by [crate::bipack_sink::BipackSink::put_var_bytes] or
    /// [crate::bipack_sink::BipackSink::put_str]. The size is encoded the same way as does
    /// [crate::bipack_sink::BipackSink::put_unsigned] and can be manually read by
    /// [BipackSource::get_unsigned].
    fn var_bytes(self: &mut Self) -> Result<Vec<u8>> {
        let size = self.get_unsigned()? as usize;
        self.get_fixed_bytes(size)
    }

    /// REad a variable length string from a source packed with
    /// [crate::bipack_sink::BipackSink::put_str]. It is a variable sized array fo utf8 encoded
    /// characters.
    fn str(self: &mut Self) -> Result<String> {
        String::from_utf8(
            self.var_bytes()?
        ).or_else(|e| Err(BipackError::BadEncoding(e)))
    }
}

/// The bipack source capable of extracting data from a slice.
/// use [SliceSource::from()] to create one.
pub struct SliceSource<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> SliceSource<'a> {
    pub fn from(src: &'a [u8]) -> SliceSource {
        SliceSource { data: src, position: 0 }
    }
}

impl<'x> BipackSource for SliceSource<'x> {
    fn get_u8(self: &mut Self) -> Result<u8> {
        if self.position >= self.data.len() {
            Err(NoDataError)
        } else {
            let result = self.data[self.position];
            self.position += 1;
            Ok(result)
        }
    }
}


