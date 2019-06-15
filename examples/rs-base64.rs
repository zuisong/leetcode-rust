fn main() {
    let word = "a";
    let bytes = word.as_bytes();
    {
        bytes.iter().for_each(|it| print!("{:08b} ", it));
        println!();
    }
    let base64 = encode_base64(bytes);
    println!("{:20} 加密后--> {:20}", word, base64);

    let bytes = decode_base64(&base64);
    {
        bytes.iter().for_each(|it| print!("{:08b} ", it));
        println!();
    }
    println!(
        "{:20} 解密后--> {:20}",
        base64,
        String::from_utf8(bytes).unwrap()
    );
}

///
///
/// base64-encode
///
/// 对二进制数据进行处理，每3个字节一组，一共是3x8=24bit，划为4组，每组正好6个bit：
///
///
/// 这样我们得到4个数字作为索引，然后查表，获得相应的4个字符，就是编码后的字符串。
///
/// 所以，Base64编码会把3字节的二进制数据编码为4字节的文本数据，长度增加33%，好处是编码后的文本数据可以在邮件正文、网页等直接显示。
///
/// 如果要编码的二进制数据不是3的倍数，最后会剩下1个或2个字节怎么办？Base64用\x00字节在末尾补足后，再在编码的末尾加上1个或2个=号，表示补了多少字节，解码的时候，会自动去掉。
///
/// Python内置的base64可以直接进行base64的编解码：
///

const BASE64_ALPHABET: [u8; 64] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', //   0 -   9
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', //  10 -  19
    b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', //  20 -  29
    b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', //  30 -  39
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', //  40 -  49
    b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', //  50 -  59
    b'8', b'9', b'+', b'/', //  60 -  63
];

const BASE64_DEALPHABET: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 0, 0, 0, 0, 0,
];

fn encode_base64(data: &[u8]) -> String {
    let len = data.len();

    let mut res: Vec<u8> = vec![];
    let mut j = 0;
    for i in 0..((len - 1) / 3 + 1) * 3 {
        match j {
            0 => {
                res.push((data.get(i).unwrap_or(&0u8) & 0b1111_1100) >> 2);
            }
            1 => {
                res.push(
                    ((data.get(i - 1).unwrap_or(&0u8) & 0b0000_0011) << 4)
                        + ((data.get(i).unwrap_or(&0u8) & 0b1111_0000) >> 4),
                );

                res.push(
                    ((data.get(i + 1).unwrap_or(&0u8) & 0b1100_0000) >> 6)
                        + ((data.get(i).unwrap_or(&0u8) & 0b0000_1111) << 2),
                );
            }
            2 => {
                res.push(data.get(i).unwrap_or(&0u8) & 0b0011_1111);
            }
            _ => unreachable!(),
        }
        if j == 2 {
            j = 0;
        } else {
            j += 1;
        }
    }
    res.iter().for_each(|it| print!("{:06b} ", it));
    println!();
    let mut res: Vec<u8> = res.iter().map(|it| BASE64_ALPHABET[*it as usize]).collect();
    let n = len % 3;
    let len = res.len();
    if n == 1 {
        res[len - 1] = b'=';
        res[len - 2] = b'=';
    }
    if n == 2 {
        res[len - 1] = b'=';
    }
    return String::from_utf8(res).unwrap();
}

fn decode_base64(base64_str: &String) -> Vec<u8> {
    if base64_str.is_empty() {
        return vec![];
    }

    let bytes = base64_str.as_bytes();

    let n1 = bytes[bytes.len() - 1] == b'=';
    let n2 = bytes[bytes.len() - 2] == b'=';

    let bytes: Vec<_> = bytes
        .iter()
        .map(|it| BASE64_DEALPHABET[*it as usize])
        .collect();

    let mut res = Vec::new();

    for i in 0..bytes.len() {
        let j = i % 4;
        let b: u8 = match j {
            0 => (bytes[i] << 2) + ((bytes[i + 1] & 0b0011_0000) >> 4),
            1 => ((bytes[i] & 0b0000_1111) << 4) + ((bytes[i + 1] & 0b0011_1100) >> 2),
            2 => ((bytes[i] & 0b0000_0011) << 6) + (bytes[i + 1] & 0b0011_1111),
            3 => {
                continue;
            }
            _ => unreachable!(),
        };

        res.push(b);
    }

    if n1 {
        res.pop();
    };
    if n2 {
        res.pop();
    };
    res
}
