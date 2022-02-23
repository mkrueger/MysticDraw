use super::TextAttribute;



pub const DOS_DEFAULT_PALETTE: [(u8, u8, u8); 16] = [
    (0x00, 0x00, 0x00), // black
    (0x00, 0x00, 0xAA), // blue
    (0x00, 0xAA, 0x00), // green
    (0x00, 0xAA, 0xAA), // cyan
    (0xAA, 0x00, 0x00), // red
    (0xAA, 0x00, 0xAA), // magenta
    (0xAA, 0x55, 0x00), // brown
    (0xAA, 0xAA, 0xAA), // lightgray
    (0x55, 0x55, 0x55), // darkgray
    (0x55, 0x55, 0xFF), // lightblue
    (0x55, 0xFF, 0x55), // lightgreen
    (0x55, 0xFF, 0xFF), // lightcyan
    (0xFF, 0x55, 0x55), // lightred
    (0xFF, 0x55, 0xFF), // lightmagenta
    (0xFF, 0xFF, 0x55), // yellow
    (0xFF, 0xFF, 0xFF), // white
];

#[derive(Clone, Copy, Debug, Default)]
pub struct DosChar {
    pub char_code: u8,
    pub attribute: TextAttribute,
}

impl DosChar {
    pub fn new() -> Self {
        DosChar {
            char_code: 0,
            attribute: super::DEFAULT_ATTRIBUTE,
        }
    }
/* 
    pub fn get_background_srgb(&self) -> (f32, f32, f32, f32) {
        let o = ((self.attribute & 0b0111_0000) >> 4) as usize;

        let c = DOS_DEFAULT_PALETTE[o];

        (
            c.0 as f32 / 255_f32,
            c.1 as f32 / 255_f32,
            c.2 as f32 / 255_f32,
            1_f32,
        )
    }*/
}
 /*
pub fn get_color(color: u8) -> &'static str
{
    match color {
        0 => "Black",
        1 => "Blue",
        2 => "Green",
        3 => "Aqua",
        4 => "Red",
        5 => "Purple",
        6 => "Brown",
        7 => "Light Gray",
        8 => "Gray",
        9 => "Light Blue",
        10 => "Light Green",
        11 => "Light Aqua",
        12 => "Light Red",
        13 => "Light Purple",
        14 => "Light Yelllow",
        15 => "White",
        _ => "Unknown"
    }
}
*/