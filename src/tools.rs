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

use string_builder::Builder;

/// Convert binary data into text dump, human readable, like:
/// ```text
/// 0000 00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f |................|
/// 0010 10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f |................|
/// 0020 20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f | !"#$%&'()*+,-./|
/// 0030 30 31                                           |01              |
///```
pub fn to_dump(data: &[u8]) -> String {
    let mut offset = 0usize;
    let mut counter = 0;
    let mut result = Builder::default();

    fn ascii_dump(result: &mut Builder, counter: usize, data: &[u8], offset: usize) {
        for i in counter..16 { result.append("   "); }
        result.append("|");
        for i in 0..counter {
            let b = data[offset - counter + i];
            if b >= 32 && b <= 127 {
                result.append(b as char)
            } else {
                result.append('.');
            }
        }
        for i in counter..16 { result.append(' '); }
        result.append("|\n");
    }

    while offset < data.len() {
        if counter == 0 {
            result.append(format!("{:04X} ", offset))
        }
        counter += 1;
        result.append(format!("{:02x} ", data[offset]));
        offset += 1;
        if counter == 16 {
            ascii_dump(&mut result, counter, data, offset);
            counter = 0;
        }
    }
    if counter != 0 { ascii_dump(&mut result, counter, data, offset); }
    result.string().unwrap()
}

