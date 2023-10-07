use string_builder::Builder;

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
