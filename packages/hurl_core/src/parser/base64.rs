/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2022 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use super::reader::Reader;

// part of hurl
// just reuse Parser/Error Position
// do not depend on external separator
// stop parsing when there is no more base64 character
//
// what kind of errors can you have?
// can only fail if using bad padding?
// if padding is used it must be used properly
// you can only have an Expecting padding error (missing one for example)

/*
https://en.wikipedia.org/wiki/Base64
Test padding/no-padding

Encoded
YW55IGNhcm5hbCBwbGVhcw==		any carnal pleas   # [97, 110, 121, 32, 99, 97, 114, 110, 97, 108, 32, 112, 108, 101, 97, 115]
*/

pub fn parse(reader: &mut Reader) -> Vec<u8> {
    let mut bytes = vec![];
    let mut buf = vec![]; // base64 text
    loop {
        let pad = padding(reader);
        if !pad.is_empty() {
            break;
        }
        let save = reader.state.clone();
        match reader.read() {
            None => {
                break;
            }
            Some(' ') | Some('\n') | Some('\t') => {}
            Some(c) => match value(c) {
                None => {
                    reader.state = save;
                    break;
                }
                Some(v) => {
                    buf.push(v);
                    if buf.len() == 4 {
                        let bs = decode_four_chars(
                            *buf.get(0).unwrap(),
                            *buf.get(1).unwrap(),
                            *buf.get(2).unwrap(),
                            *buf.get(3).unwrap(),
                        );
                        for b in bs {
                            bytes.push(b);
                        }
                        buf = vec![];
                    }
                }
            },
        }
    }
    match buf.as_slice() {
        [c1, c2] => bytes.append(&mut decode_two_chars(*c1, *c2)),
        [c1, c2, c3] => bytes.append(&mut decode_three_chars(*c1, *c2, *c3)),
        _ => {}
    }
    bytes
}

fn value(c: char) -> Option<i32> {
    match c {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(2),
        'D' => Some(3),
        'E' => Some(4),
        'F' => Some(5),
        'G' => Some(6),
        'H' => Some(7),
        'I' => Some(8),
        'J' => Some(9),
        'K' => Some(10),
        'L' => Some(11),
        'M' => Some(12),
        'N' => Some(13),
        'O' => Some(14),
        'P' => Some(15),
        'Q' => Some(16),
        'R' => Some(17),
        'S' => Some(18),
        'T' => Some(19),
        'U' => Some(20),
        'V' => Some(21),
        'W' => Some(22),
        'X' => Some(23),
        'Y' => Some(24),
        'Z' => Some(25),
        'a' => Some(26),
        'b' => Some(27),
        'c' => Some(28),
        'd' => Some(29),
        'e' => Some(30),
        'f' => Some(31),
        'g' => Some(32),
        'h' => Some(33),
        'i' => Some(34),
        'j' => Some(35),
        'k' => Some(36),
        'l' => Some(37),
        'm' => Some(38),
        'n' => Some(39),
        'o' => Some(40),
        'p' => Some(41),
        'q' => Some(42),
        'r' => Some(43),
        's' => Some(44),
        't' => Some(45),
        'u' => Some(46),
        'v' => Some(47),
        'w' => Some(48),
        'x' => Some(49),
        'y' => Some(50),
        'z' => Some(51),
        '0' => Some(52),
        '1' => Some(53),
        '2' => Some(54),
        '3' => Some(55),
        '4' => Some(56),
        '5' => Some(57),
        '6' => Some(58),
        '7' => Some(59),
        '8' => Some(60),
        '9' => Some(61),
        '+' => Some(62),
        '/' => Some(63),
        _ => None,
    }
}

fn padding(reader: &mut Reader) -> String {
    // consume padding can not fail
    let mut buf = String::from("");
    loop {
        let save = reader.state.clone();
        match reader.read() {
            Some('=') => {
                buf.push('=');
            }
            _ => {
                reader.state = save;
                break;
            }
        }
    }
    buf
}

fn decode_two_chars(c1: i32, c2: i32) -> Vec<u8> {
    return vec![((c1 << 2 & 255) + (c2 >> 4)) as u8];
}

fn decode_three_chars(c1: i32, c2: i32, c3: i32) -> Vec<u8> {
    return vec![
        ((c1 << 2 & 255) + (c2 >> 4)) as u8,
        ((c2 << 4 & 255) + (c3 >> 2)) as u8,
    ];
}

fn decode_four_chars(c1: i32, c2: i32, c3: i32, c4: i32) -> Vec<u8> {
    return vec![
        ((c1 << 2 & 255) + (c2 >> 4)) as u8,
        ((c2 << 4 & 255) + (c3 >> 2)) as u8,
        (((c3 << 6) & 255) + c4) as u8,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_one_block() {
        let mut reader = Reader::init("");
        assert_eq!(parse(&mut reader), vec![] as Vec<u8>);
        assert_eq!(reader.state.cursor, 0);

        let mut reader = Reader::init("AA==;");
        assert_eq!(parse(&mut reader), vec![0]);
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("AA");
        assert_eq!(parse(&mut reader), vec![0]);
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("AA;");
        assert_eq!(parse(&mut reader), vec![0]);
        assert_eq!(reader.state.cursor, 2);

        let mut reader = Reader::init("TWE=;");
        assert_eq!(parse(&mut reader), vec![77, 97]);
        assert_eq!(reader.state.cursor, 4);

        let mut reader = Reader::init("TWFu;");
        assert_eq!(parse(&mut reader), vec![77, 97, 110]);
        assert_eq!(reader.state.cursor, 4);
    }

    /*
    |   Y       |     W     |     5     |     5     |
    |     24    |    22     |      57   |     57    |
    |0|1|1|0|0|0|0|1|0|1|1|0|1|1|1|0|0|1|1|1|1|0|0|1|
    |      97       |     110       |      121      |
    */

    /*
    |   Y       |     W     |     5     |     5     |
    |     24    |    22     |      57   |     57    |
    |0|1|1|0|0|0|0|1|0|1|1|0|1|1|1|0|0|1|1|1|1|0|0|1|
    |      97       |     110       |      121      |
    */

    #[test]
    fn test_decode_with_padding() {
        let mut reader = Reader::init("YW55IGNhcm5hbCBwbGVhcw==;");
        let decoded = parse(&mut reader);
        assert_eq!(decoded, b"any carnal pleas");

        let mut reader = Reader::init("YW55IGNhcm5hbCBwbGVhc3U=;");
        assert_eq!(parse(&mut reader), b"any carnal pleasu");

        let mut reader = Reader::init("YW55IGNhcm5hbCBwbGVhc3Vy;");
        assert_eq!(parse(&mut reader), b"any carnal pleasur");
    }

    #[test]
    fn test_decode_without_padding() {
        let mut reader = Reader::init("YW55IGNhcm5hbCBwbGVhcw;");
        assert_eq!(parse(&mut reader), b"any carnal pleas");

        let mut reader = Reader::init("YW55IGNhcm5hbCBwbGVhc3U;");
        assert_eq!(parse(&mut reader), b"any carnal pleasu");
    }

    #[test]
    fn test_decode_with_whitespace() {
        let mut reader = Reader::init("TW E=\n;");
        assert_eq!(parse(&mut reader), vec![77, 97]);
        assert_eq!(reader.state.cursor, 5);
    }

    #[test]
    fn test_decode_two_chars() {
        assert_eq!(
            decode_two_chars(value('A').unwrap(), value('A').unwrap()),
            vec![0]
        );
        assert_eq!(
            decode_two_chars(value('A').unwrap(), value('Q').unwrap()),
            vec![1]
        );
        assert_eq!(
            decode_two_chars(value('T').unwrap(), value('Q').unwrap()),
            vec![77]
        );
    }

    #[test]
    fn test_decode_three_chars() {
        assert_eq!(
            decode_three_chars(
                value('T').unwrap(),
                value('W').unwrap(),
                value('E').unwrap(),
            ),
            vec![77, 97]
        );
    }

    #[test]
    fn test_decode_four_chars() {
        assert_eq!(
            decode_four_chars(
                value('Y').unwrap(),
                value('W').unwrap(),
                value('5').unwrap(),
                value('5').unwrap(),
            ),
            vec![97, 110, 121]
        );
        assert_eq!(
            decode_four_chars(
                value('T').unwrap(),
                value('W').unwrap(),
                value('F').unwrap(),
                value('u').unwrap(),
            ),
            vec![77, 97, 110]
        );
    }
}
