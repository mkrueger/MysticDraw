#![allow(dead_code)]

use crate::model::BitFont;
use std::io;

use super::Buffer;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum SauceDataType {
    /// Undefined filetype.
    /// You could use this to add SAUCE to a custom or proprietary file, without giving it any particular meaning or interpretation.
    Undefined = 0,

    /// A character based file.
    /// These files are typically interpreted sequentially. Also known as streams.  
    Character = 1,

    /// Bitmap graphic and animation files.
    Bitmap = 2,

    /// A vector graphic file.
    Vector = 3,

    /// An audio file.
    Audio = 4,

    /// This is a raw memory copy of a text mode screen. Also known as a .BIN file.
    /// This is essentially a collection of character and attribute pairs.
    BinaryText = 5,

    /// An XBin or eXtended BIN file.
    XBin = 6,

    /// An archive file.
    Archive = 7,

    ///  A executable file.
    Executable = 8,
}

impl SauceDataType {
    pub fn from(b: u8) -> SauceDataType {
        match b {
            0 => SauceDataType::Undefined,
            1 => SauceDataType::Character,
            2 => SauceDataType::Bitmap,
            3 => SauceDataType::Vector,
            4 => SauceDataType::Audio,
            5 => SauceDataType::BinaryText,
            6 => SauceDataType::XBin,
            7 => SauceDataType::Archive,
            8 => SauceDataType::Executable,
            _ => {
                eprintln!("unknown sauce data type {}", b);
                SauceDataType::Undefined
            }
        }
    }
}

impl Default for SauceDataType {
    fn default() -> SauceDataType {
        SauceDataType::Undefined
    }
}

const SAUCE_SIZE: i32 = 128;

pub enum SauceFileType {
    Undefined,
    Ascii,
    Ansi,
    ANSiMation,
    PCBoard,
    Avatar,
    TundraDraw,
    Bin,
    XBin,
}

const SAUCE_FILE_TYPE_ASCII: u8 = 0;
const SAUCE_FILE_TYPE_ANSI: u8 = 1;
const SAUCE_FILE_TYPE_ANSIMATION: u8 = 2;
const SAUCE_FILE_TYPE_PCBOARD: u8 = 4;
const SAUCE_FILE_TYPE_AVATAR: u8 = 5;
const SAUCE_FILE_TYPE_TUNDRA_DRAW: u8 = 8;

#[derive(Clone, Default)]
pub struct SauceString<const LEN: usize, const EMPTY: u8>(Vec<u8>);

pub const CP437_TO_UNICODE: [char; 256] = [
    '\u{0000}', '\u{263a}', '\u{263b}', '\u{2665}', '\u{2666}', '\u{2663}', '\u{2660}', '\u{2022}',
    '\u{25d8}', '\u{25cb}', '\u{25d9}', '\u{2642}', '\u{2640}', '\u{266a}', '\u{266b}', '\u{263c}',
    '\u{25ba}', '\u{25c4}', '\u{2195}', '\u{203c}', '\u{00b6}', '\u{00a7}', '\u{25ac}', '\u{21a8}',
    '\u{2191}', '\u{2193}', '\u{2192}', '\u{2190}', '\u{221f}', '\u{2194}', '\u{25b2}', '\u{25bc}',
    '\u{0020}', '\u{0021}', '\u{0022}', '\u{0023}', '\u{0024}', '\u{0025}', '\u{0026}', '\u{0027}',
    '\u{0028}', '\u{0029}', '\u{002a}', '\u{002b}', '\u{002c}', '\u{002d}', '\u{002e}', '\u{002f}',
    '\u{0030}', '\u{0031}', '\u{0032}', '\u{0033}', '\u{0034}', '\u{0035}', '\u{0036}', '\u{0037}',
    '\u{0038}', '\u{0039}', '\u{003a}', '\u{003b}', '\u{003c}', '\u{003d}', '\u{003e}', '\u{003f}',
    '\u{0040}', '\u{0041}', '\u{0042}', '\u{0043}', '\u{0044}', '\u{0045}', '\u{0046}', '\u{0047}',
    '\u{0048}', '\u{0049}', '\u{004a}', '\u{004b}', '\u{004c}', '\u{004d}', '\u{004e}', '\u{004f}',
    '\u{0050}', '\u{0051}', '\u{0052}', '\u{0053}', '\u{0054}', '\u{0055}', '\u{0056}', '\u{0057}',
    '\u{0058}', '\u{0059}', '\u{005a}', '\u{005b}', '\u{005c}', '\u{005d}', '\u{005e}', '\u{005f}',
    '\u{0060}', '\u{0061}', '\u{0062}', '\u{0063}', '\u{0064}', '\u{0065}', '\u{0066}', '\u{0067}',
    '\u{0068}', '\u{0069}', '\u{006a}', '\u{006b}', '\u{006c}', '\u{006d}', '\u{006e}', '\u{006f}',
    '\u{0070}', '\u{0071}', '\u{0072}', '\u{0073}', '\u{0074}', '\u{0075}', '\u{0076}', '\u{0077}',
    '\u{0078}', '\u{0079}', '\u{007a}', '\u{007b}', '\u{007c}', '\u{007d}', '\u{007e}', '\u{007f}',
    '\u{00c7}', '\u{00fc}', '\u{00e9}', '\u{00e2}', '\u{00e4}', '\u{00e0}', '\u{00e5}', '\u{00e7}',
    '\u{00ea}', '\u{00eb}', '\u{00e8}', '\u{00ef}', '\u{00ee}', '\u{00ec}', '\u{00c4}', '\u{00c5}',
    '\u{00c9}', '\u{00e6}', '\u{00c6}', '\u{00f4}', '\u{00f6}', '\u{00f2}', '\u{00fb}', '\u{00f9}',
    '\u{00ff}', '\u{00d6}', '\u{00dc}', '\u{00a2}', '\u{00a3}', '\u{00a5}', '\u{20a7}', '\u{0192}',
    '\u{00e1}', '\u{00ed}', '\u{00f3}', '\u{00fa}', '\u{00f1}', '\u{00d1}', '\u{00aa}', '\u{00ba}',
    '\u{00bf}', '\u{2310}', '\u{00ac}', '\u{00bd}', '\u{00bc}', '\u{00a1}', '\u{00ab}', '\u{00bb}',
    '\u{2591}', '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}',
    '\u{2555}', '\u{2563}', '\u{2551}', '\u{2557}', '\u{255d}', '\u{255c}', '\u{255b}', '\u{2510}',
    '\u{2514}', '\u{2534}', '\u{252c}', '\u{251c}', '\u{2500}', '\u{253c}', '\u{255e}', '\u{255f}',
    '\u{255a}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}', '\u{2550}', '\u{256c}', '\u{2567}',
    '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}', '\u{2558}', '\u{2552}', '\u{2553}', '\u{256b}',
    '\u{256a}', '\u{2518}', '\u{250c}', '\u{2588}', '\u{2584}', '\u{258c}', '\u{2590}', '\u{2580}',
    '\u{03b1}', '\u{00df}', '\u{0393}', '\u{03c0}', '\u{03a3}', '\u{03c3}', '\u{00b5}', '\u{03c4}',
    '\u{03a6}', '\u{0398}', '\u{03a9}', '\u{03b4}', '\u{221e}', '\u{03c6}', '\u{03b5}', '\u{2229}',
    '\u{2261}', '\u{00b1}', '\u{2265}', '\u{2264}', '\u{2320}', '\u{2321}', '\u{00f7}', '\u{2248}',
    '\u{00b0}', '\u{2219}', '\u{00b7}', '\u{221a}', '\u{207f}', '\u{00b2}', '\u{25a0}', '\u{00a0}',
];

impl<const LEN: usize, const EMPTY: u8> std::fmt::Display for SauceString<LEN, EMPTY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let len = self.len();
        for i in 0..len {
            let b = self.0[i];
            str.push(CP437_TO_UNICODE[b as usize]);
        }
        write!(f, "{}", str)
    }
}

impl<const LEN: usize, const EMPTY: u8> PartialEq for SauceString<LEN, EMPTY> {
    fn eq(&self, other: &Self) -> bool {
        let l1 = self.len();
        let l2 = other.len();

        if l1 != l2 {
            return false;
        }

        self.0[0..l1] == other.0[0..l2]
    }
}

impl<const LEN: usize, const EMPTY: u8> std::fmt::Debug for SauceString<LEN, EMPTY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(SauceString<{}> {})",
            LEN,
            String::from_utf8_lossy(&self.0)
        )
    }
}

impl<const LEN: usize, const EMPTY: u8> SauceString<LEN, EMPTY> {
    pub const EMPTY: SauceString<LEN, EMPTY> = SauceString(Vec::new());

    pub fn new() -> Self {
        SauceString(Vec::new())
    }

    pub fn from(str: &str) -> Self {
        let mut data = Vec::new();
        for ch in str.chars() {
            if data.len() >= LEN {
                break;
            }
            let mut found = false;
            #[allow(clippy::needless_range_loop)]
            for i in 0..CP437_TO_UNICODE.len() {
                if ch == CP437_TO_UNICODE[i] {
                    data.push(i as u8);
                    found = true;
                    break;
                }
            }
            if !found {
                data.push(b'?');
            }
        }
        SauceString(data)
    }

    pub fn len(&self) -> usize {
        let mut len = self.0.len();
        while len > 0 {
            let ch = self.0[len - 1];
            if ch != 0 && ch != b' ' {
                break;
            }
            len -= 1;
        }
        len
    }

    #[allow(clippy::unused_self)]
    pub fn max_len(&self) -> usize {
        LEN
    }

    pub fn read(&mut self, data: &[u8]) -> usize {
        let mut last_non_empty = LEN;
        #[allow(clippy::needless_range_loop)]
        for i in 0..LEN {
            if EMPTY == 0 && data[i] == 0 {
                break;
            }
            if data[i] != EMPTY {
                last_non_empty = i + 1;
            }
            self.0.push(data[i]);
        }
        if last_non_empty < LEN {
            self.0.truncate(last_non_empty);
        }
        LEN
    }

    pub fn append_to(&self, vec: &mut Vec<u8>) {
        vec.extend(&self.0);
        if self.0.len() < LEN {
            vec.resize(vec.len() + LEN - self.0.len(), EMPTY);
        }
    }
}

/// | Field    | Type | Size | Descritption
/// |----------|------|------|-------------
/// | ID       | char | 5    | SAUCE comment block ID. This should be equal to "COMNT".
/// | Line 1   | char | 64   | Line of text.
/// | ...      |      |      |
/// | Line n   | char | 64   | Last line of text
const SAUCE_COMMENT_ID: [u8; 5] = *b"COMNT";
const SAUCE_ID: [u8; 5] = *b"SAUCE";
const SAUCE_LEN: usize = 128;
const ANSI_FLAG_NON_BLINK_MODE: u8 = 1;
static EMPTY_TINFO: SauceString<22, 0> = SauceString(Vec::new());

impl Buffer {
    pub fn read_sauce_info(&mut self, data: &[u8]) -> io::Result<(SauceFileType, usize)> {
        if data.len() < SAUCE_LEN {
            return Ok((SauceFileType::Undefined, data.len()));
        }

        let mut o = data.len() - SAUCE_LEN;
        if SAUCE_ID != data[o..(o + 5)] {
            return Ok((SauceFileType::Undefined, data.len()));
        }
        o += 5;

        if b"00" != &data[o..(o + 2)] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Unsupported sauce version {}{}",
                    char::from_u32(data[o + 5] as u32).unwrap(),
                    char::from_u32(data[o + 6] as u32).unwrap()
                )
                .as_str(),
            ));
        }
        o += 2;
        o += self.title.read(&data[o..]);
        o += self.author.read(&data[o..]);
        o += self.group.read(&data[o..]);

        // skip date
        o += 8;

        // skip file_size - we can calculate it, better than to rely on random 3rd party software.
        // Question: are there files where that is important?
        o += 4;

        let data_type = SauceDataType::from(data[o]);
        o += 1;
        let file_type = data[o];
        o += 1;
        let t_info1 = data[o] as u16 + ((data[o + 1] as u16) << 8);
        o += 2;
        let t_info2 = data[o] as u16 + ((data[o + 1] as u16) << 8);
        o += 2;
        // let t_info3 = data[o] as u16 + ((data[o + 1] as u16) << 8);
        o += 2;
        // let t_info4 = data[o] as u16 + ((data[o + 1] as u16) << 8);
        o += 2;
        let num_comments: u8 = data[o];
        o += 1;
        let t_flags: u8 = data[o];
        o += 1;
        let mut t_info_str: SauceString<22, 0> = SauceString::new();
        o += t_info_str.read(&data[o..]);
        assert_eq!(data.len(), o);

        let mut sauce_file_type = SauceFileType::Undefined;

        match data_type {
            SauceDataType::BinaryText => {
                self.width = (file_type as u16) << 1;
                sauce_file_type = SauceFileType::Bin;
                self.use_ice = (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                self.font = BitFont::from_name(&t_info_str.to_string()).unwrap_or_default();
            }
            SauceDataType::XBin => {
                self.width = t_info1;
                self.height = t_info2;
                sauce_file_type = SauceFileType::XBin;
                // no flags according to spec
            }
            SauceDataType::Character => {
                match file_type {
                    SAUCE_FILE_TYPE_ASCII => {
                        self.width = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::Ascii;
                        self.use_ice =
                            (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                        self.font = BitFont::from_name(&t_info_str.to_string()).unwrap_or_default();
                    }
                    SAUCE_FILE_TYPE_ANSI => {
                        self.width = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::Ansi;
                        self.use_ice =
                            (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                        self.font = BitFont::from_name(&t_info_str.to_string()).unwrap_or_default();
                    }
                    SAUCE_FILE_TYPE_ANSIMATION => {
                        self.width = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::ANSiMation;
                        self.use_ice =
                            (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                        self.font = BitFont::from_name(&t_info_str.to_string()).unwrap_or_default();
                    }
                    SAUCE_FILE_TYPE_PCBOARD => {
                        self.width = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::PCBoard;
                        // no flags according to spec
                    }
                    SAUCE_FILE_TYPE_AVATAR => {
                        self.width = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::Avatar;
                        // no flags according to spec
                    }
                    SAUCE_FILE_TYPE_TUNDRA_DRAW => {
                        self.width = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::TundraDraw;
                        // no flags according to spec
                    }
                    _ => {}
                }
            }
            _ => {
                eprintln!(
                    "useless/invalid sauce info data type: {data_type:?} file type: {file_type}."
                );
            }
        }

        if num_comments == 0 {
            Ok((sauce_file_type, data.len() - SAUCE_LEN - 1)) // -1 is from the EOF char
        } else if (data.len() - SAUCE_LEN) as i32 - num_comments as i32 * 64 - 5 < 0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid sauce comment block",
            ))
        } else {
            let comment_start = (data.len() - SAUCE_LEN) - num_comments as usize * 64 - 5;
            o = comment_start;
            if SAUCE_COMMENT_ID != data[o..(o + 5)] {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid SAUCE comment id {}",
                        String::from_utf8_lossy(&data[o..(o + 5)])
                    ),
                ));
            }
            o += 5;
            for _ in 0..num_comments {
                let mut comment: SauceString<64, 0> = SauceString::new();
                o += comment.read(&data[o..]);
                self.comments.push(comment);
            }
            Ok((sauce_file_type, comment_start - 1)) // -1 is from the EOF char
        }
    }

    pub fn write_sauce_info(
        &self,
        sauce_file_type: &SauceFileType,
        vec: &mut Vec<u8>,
    ) -> io::Result<bool> {
        vec.push(0x1A); // EOF Char.
        let file_size = vec.len() as u32;
        if !self.comments.is_empty() {
            if self.comments.len() > 255 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "sauce comments exceed maximum of 255: {}.",
                        self.comments.len()
                    )
                    .as_str(),
                ));
            }
            vec.extend(SAUCE_COMMENT_ID);
            for cmt in &self.comments {
                cmt.append_to(vec);
            }
        }

        vec.extend(SAUCE_ID);
        vec.push(b'0');
        vec.push(b'0');
        self.title.append_to(vec);
        self.author.append_to(vec);
        self.group.append_to(vec);
        // TODO: Dates
        vec.extend(b"20130504");
        vec.extend(u32::to_le_bytes(file_size));

        let data_type;
        let file_type;
        let mut t_info1 = 0;
        let mut t_info2 = 0;
        let t_info3 = 0;
        let t_info4 = 0;
        let mut t_flags = 0;
        let mut t_info_str = &self.font.name;

        match sauce_file_type {
            SauceFileType::Ascii => {
                data_type = SauceDataType::Character;
                file_type = SAUCE_FILE_TYPE_ASCII;
                t_info1 = self.width;
                t_info2 = self.height;
                if self.use_ice { t_flags |= ANSI_FLAG_NON_BLINK_MODE; }
            },
            SauceFileType::Undefined | // map everything else just to ANSI
            SauceFileType::Ansi => {
                data_type = SauceDataType::Character;
                file_type = SAUCE_FILE_TYPE_ANSI;
                t_info1 = self.width;
                t_info2 = self.height;
                if self.use_ice { t_flags |= ANSI_FLAG_NON_BLINK_MODE; }
            },
            SauceFileType::ANSiMation => {
                data_type = SauceDataType::Character;
                file_type = SAUCE_FILE_TYPE_ANSIMATION;
                t_info1 = self.width;
                t_info2 = self.height;
                if self.use_ice { t_flags |= ANSI_FLAG_NON_BLINK_MODE; }
            },
            SauceFileType::PCBoard => {
                data_type = SauceDataType::Character;
                file_type = SAUCE_FILE_TYPE_PCBOARD;
                t_info1 = self.width;
                t_info2 = self.height;
                // no flags
                t_info_str = &EMPTY_TINFO;
            },
            SauceFileType::Avatar => {
                data_type = SauceDataType::Character;
                file_type = SAUCE_FILE_TYPE_AVATAR;
                t_info1 = self.width;
                t_info2 = self.height;
                // no flags
                t_info_str = &EMPTY_TINFO;
            },
            SauceFileType::TundraDraw => {
                data_type = SauceDataType::Character;
                file_type = SAUCE_FILE_TYPE_TUNDRA_DRAW;
                t_info1 = self.width;
                // no flags
                t_info_str = &EMPTY_TINFO;
            }
            SauceFileType::Bin => {
                data_type = SauceDataType::BinaryText;
                let w = self.width / 2;
                if w > u8::MAX as u16 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "BIN files can only be saved up to 510 width."));
                }
                file_type = w as u8;
                if self.use_ice { t_flags |= ANSI_FLAG_NON_BLINK_MODE; }
            },
            SauceFileType::XBin => {
                data_type = SauceDataType::XBin;
                file_type = 0;
                t_info1 = self.width;
                t_info2 = self.height;
                // no flags
                t_info_str = &EMPTY_TINFO;
            }
        }

        vec.push(data_type as u8);
        vec.push(file_type);
        vec.extend(u16::to_le_bytes(t_info1));
        vec.extend(u16::to_le_bytes(t_info2));
        vec.extend(u16::to_le_bytes(t_info3));
        vec.extend(u16::to_le_bytes(t_info4));
        vec.push(self.comments.len() as u8); // comment len is checked above for <= 255
        vec.push(t_flags);
        t_info_str.append_to(vec);

        Ok(true)
    }
}



#[cfg(test)]
mod tests {
    use super::SauceString;

    #[test]
    fn test_sauce_string_string_conversion() {

        let str = SauceString::<20, 0>::from("Hello World!");
        assert_eq!("Hello World!", str.to_string());
    }
}
