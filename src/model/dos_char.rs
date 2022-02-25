use super::TextAttribute;

#[derive(Clone, Copy, Debug, Default)]
pub struct DosChar {
    pub char_code: u8,
    pub attribute: TextAttribute,
}

impl DosChar {
    pub fn new() -> Self {
        DosChar {
            char_code: 0,
            attribute: super::TextAttribute::DEFAULT,
        }
    }
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