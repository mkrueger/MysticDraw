#![allow(dead_code)]

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum SauceDataType {
    /// Undefined filetype.
    /// You could use this to add SAUCE to a custom or proprietary file, without giving it any particular meaning or interpretation. 
    None = 0,
    
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
    Excutable = 8
}

impl Default for SauceDataType {
    fn default() -> SauceDataType {
        SauceDataType::None
    }
}

/// | Field    | Type | Size | Descritption
/// |----------|------|------|-------------
/// | ID       | char | 5    | SAUCE comment block ID. This should be equal to "COMNT".
/// | Line 1   | char | 64   | Line of text.
/// | ...      |      |      | 
/// | Line n   | char | 64   | Last line of text
#[derive(Clone, Debug, Default)]
pub struct SauceCommentBlock {
    lines: Vec<String>
}

const SAUCE_SIZE : i32 = 128;

/// SAUCE – Standard Architecture for Universal Comment Extensions
/// http://www.acid.org/info/sauce/sauce.htm
///
/// | Field    | Type | Size | Descritption
/// |----------|------|------|-------------
/// | ID       | char | 5    | SAUCE identification. This should be equal to "SAUCE".
/// | Version  | char | 2    | SAUCE version number, should be "00".
/// | Title    | char | 35   | The title of the file. 
/// | Author   | char | 20   | The (nick)name or handle of the creator of the file. 
/// | Group    | char | 20   | The name of the group or company the creator is employed by. 
/// | Date     | char | 8    | The format for the date is CCYYMMDD (century, year, month, day).
/// | FileSize | u32  | 4    | The original file size not including the SAUCE information. 
/// | DataType | u8   | 1    | Type of data. 
/// | FileType | u8   | 1    | Type of file. 
/// | TInfo1   | u16  | 2    | Type dependant numeric information field 1. 
/// | TInfo2   | u16  | 2    | Type dependant numeric information field 2. 
/// | TInfo3   | u16  | 2    | Type dependant numeric information field 3. 
/// | TInfo4   | u16  | 2    | Type dependant numeric information field 4. 
/// | Comments | u8   | 1    | #lines in the extra SAUCE comment block. 0 indicates no comment block is present. 
/// | TFlags   | u8   | 1    | Type dependant flags. 
/// | TINfoS   | zstr | 22   | Type dependant string information field 
/// 
/// char type: should be filled with spaces if unused, COULD be terminated with 0
/// zstr: c like string where all unused space should be filled with 0
#[derive(Clone, Debug, Default)]
pub struct Sauce {
    title: String,
    author: String,
    group: String,
    date: String,
    data_type: SauceDataType,   

    /// SAUCE FileType
    /// (I only inserted the Character file types here, the others are not relevant for that tool.)
    /// 
    /// | DataType | FileType | Name       | Description                        | TInfo1 | TInfo2 | TInfo3 | TInfo4 | Flags | TInfoS
    /// |----------|----------|------------|------------------------------------|--------|--------|--------|--------|-------|-------
    /// | None     | 0        | -          | Undefined                          | 0      | 0      | 0      | 0      | 0     | 0
    /// | Character| 0        | ASCII      | Plain ASCII                        | #lines | 0      | 0      | 0      | ANSI  | FontName
    /// | Character| 1        | ANSI       | full ANSI                          | Character width| #lines | 0 | 0 |  ANSI  | FontName
    /// | Character| 2        | ANSiMation | fixed size ANSI                    | Character width| #lines | 0 | 0 |  ANSI  | FontName
    /// | Character| 3        | RIP script | Remote Imaging Protocol graphics.  | Pixel width| Pixel Height | #colors | 0 |  0  | 0
    /// | Character| 4        | PCBoard    | PCB File                           | Character width| #lines | 0 | 0 |  ANSI  | FontName
    /// | Character| 5        | Avatar     | Avatar color codes                 | Character width| #lines | 0 | 0 |  ANSI  | FontName
    /// | Character| 6        | HTML       | HyperText Markup Language          | 0      | 0      | 0      | 0      | 0     | 0
    /// | Character| 7        | Source     | Any source code file               | 0      | 0      | 0      | 0      | 0     | 0
    /// | Character| 8        | TundraDraw | A TundraDraw file (custom palette) | Character width| #lines | 0 | 0 |  ANSI  | FontName
    /// 
    /// ANSI Flag
    /// 0|0|0|A|R|L|S|B
    /// B - Non Blink mode (iCE Color)
    /// LS - Letetr spacing (8/9 px font selection)
    /// 00: Legacy value. No preference.
    /// 01: Select 8 pixel font.
    /// 10: Select 9 pixel font.
    /// 11: Not currently a valid value.
    /// 
    /// AR- Aspect Ratio
    /// 00: Legacy value. No preference.
    /// 01: Image was created for a legacy device. When displayed on a device with square pixels, either the font or the image needs to be stretched.
    /// 10: Image was created for a modern device with square pixels. No stretching is desired on a device with square pixels.
    /// 11: Not currently a valid value.
    file_type: u8,
    t_info1: u16,
    t_info2: u16,
    t_info3: u16,
    t_info4: u16,
    comments: Option<SauceCommentBlock>,
    t_flags: u8,
    t_infos: String
}

const SAUCE_ID : [u8;5] = *b"SAUCE";
const SAUCE_COMMENT_ID : [u8;5] = *b"COMNT";

