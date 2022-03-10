#![allow(dead_code)]

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
    Executable = 8
}

impl SauceDataType
{
    pub fn from(b: u8) -> SauceDataType
    {
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


const SAUCE_SIZE : i32 = 128;

pub enum SauceFileType {
    Undefined,
    Ascii,
    Ansi,
    ANSiMation,
    PCBoard,
    Avatar,
    TundraDraw,
    Bin,
    XBin
}

const SAUCE_FILE_TYPE_ASCII: u8 = 0;
const SAUCE_FILE_TYPE_ANSI: u8 = 1;
const SAUCE_FILE_TYPE_ANSIMATION: u8 = 2;
const SAUCE_FILE_TYPE_PCBOARD: u8 = 4;
const SAUCE_FILE_TYPE_AVATAR: u8 = 5;
const SAUCE_FILE_TYPE_TUNDRA_DRAW: u8 = 8;

#[derive(Clone, Default)]
pub struct SauceString<const LEN: usize, const EMPTY: u8>(Vec::<u8>);

impl<const LEN: usize, const EMPTY: u8> std::fmt::Debug for SauceString<LEN, EMPTY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(SauceString<{}> {})", LEN, String::from_utf8_lossy(&self.0))
    }
}

impl<const LEN: usize, const EMPTY: u8> std::fmt::Display for SauceString<LEN, EMPTY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl<const LEN: usize, const EMPTY: u8> SauceString<LEN, EMPTY> {
    pub fn new() -> Self {
        SauceString(Vec::new())
    }

    pub fn max_len(&self) -> usize {
        LEN
    }

    pub fn read(&mut self, data: &[u8]) -> usize
    {
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

    pub fn append_to(&self, vec: &mut Vec::<u8>) {
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
const SAUCE_COMMENT_ID: [u8;5] = *b"COMNT";
const SAUCE_ID: [u8;5] = *b"SAUCE";
const SAUCE_LEN: usize = 128;
const ANSI_FLAG_NON_BLINK_MODE: u8 = 1;
static EMPTY_TINFO: SauceString::<22, 0> = SauceString(Vec::new());

impl Buffer {
    pub fn read_sauce_info(&mut self, data: &[u8]) -> io::Result<(SauceFileType, usize)>
    {
        if data.len() < SAUCE_LEN {
            return Ok((SauceFileType::Undefined, data.len()));
        }

        let mut o = data.len() - SAUCE_LEN;
        if SAUCE_ID != data[o..(o + 5)] {
            return Ok((SauceFileType::Undefined, data.len()));
        }
        o += 5;

        if b"00" != &data[o..(o + 2)] {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unsupported sauce version {}{}", char::from_u32(data[o + 5] as u32).unwrap(), char::from_u32(data[o + 6] as u32).unwrap()).as_str()));
        }
        self.write_sauce = true;
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
        let t_flags : u8 = data[o];
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
            }, 
            SauceDataType::XBin => {
                self.width  = t_info1;
                self.height = t_info2;
                sauce_file_type = SauceFileType::XBin;
                // no flags according to spec
            },
            SauceDataType::Character => {
                match file_type {
                    SAUCE_FILE_TYPE_ASCII=> {
                        self.width  = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::Ascii;
                        self.use_ice = (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                    }
                    SAUCE_FILE_TYPE_ANSI=> {
                        self.width  = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::Ansi;
                        self.use_ice = (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                    }
                    SAUCE_FILE_TYPE_ANSIMATION=> {
                        self.width  = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::ANSiMation;
                        self.use_ice = (t_flags & ANSI_FLAG_NON_BLINK_MODE) == ANSI_FLAG_NON_BLINK_MODE;
                    }
                    SAUCE_FILE_TYPE_PCBOARD=> {
                        self.width  = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::PCBoard;
                        // no flags according to spec
                    }
                    SAUCE_FILE_TYPE_AVATAR=> {
                        self.width  = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::Avatar;
                        // no flags according to spec
                    }
                    SAUCE_FILE_TYPE_TUNDRA_DRAW => {
                        self.width  = t_info1;
                        self.height = t_info2;
                        sauce_file_type = SauceFileType::TundraDraw;
                        // no flags according to spec
                    }
                    _ => {}
                }
            },
            _ => { eprintln!("useless/invalid sauce info data type: {data_type:?} file type: {file_type}.");}
        }

        if num_comments == 0 {
            Ok((sauce_file_type, data.len() - SAUCE_LEN - 1)) // -1 is from the EOF char
        } else if (data.len() - SAUCE_LEN) as i32 - num_comments as i32 * 64 - 5 < 0 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "invalid sauce comment block"))
        } else {
            let comment_start= (data.len() - SAUCE_LEN) - num_comments as usize * 64 - 5;
            o = comment_start;
            if SAUCE_COMMENT_ID != data[o..(o + 5)] {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid SAUCE comment id {}", String::from_utf8_lossy(&data[o..(o + 5)]))));
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

    pub fn write_sauce_info(&self, sauce_file_type: &SauceFileType, vec: &mut Vec<u8>) -> io::Result<bool> {
        vec.push(0x1A); // EOF Char.
        let file_size = vec.len() as u32;
        if !self.comments.is_empty() {
            if self.comments.len() > 255 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, format!("sauce comments exceed maximum of 255: {}.", self.comments.len()).as_str()));
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
        let mut t_info1= 0;
        let mut t_info2= 0;
        let t_info3= 0;
        let t_info4= 0;
        let mut t_flags= 0;
        let mut t_info_str = if let Some(name) = &self.font_name {
            name
        } else if let Some(font) = &self.font {
            &font.name
        } else {
            &EMPTY_TINFO
        };

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