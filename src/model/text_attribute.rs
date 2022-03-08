
#[derive(Clone, Copy, Debug, Default)]
pub struct TextAttribute {
    foreground_color: u8,
    background_color: u8
}
impl std::fmt::Display for TextAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Attr: {:X}, fg {}, bg {}, blink {})", self.as_u8(), self.get_foreground(), self.get_background(), self.is_blink())
    }
}

impl TextAttribute
{
    pub const DEFAULT : TextAttribute = TextAttribute{ foreground_color: 7, background_color: 0 };

    pub fn from_u8(attr: u8) -> Self
    {
        TextAttribute { foreground_color: attr & 0b0000_1111, background_color: attr >> 4 }
    }

    pub fn from_color(fg: u8, bg: u8) -> Self
    {
        TextAttribute { foreground_color: fg, background_color: bg }
    }

    pub fn as_u8(self) -> u8
    {
        self.foreground_color & 0xF | ((self.background_color & 0xF) << 4)
    }

    pub fn is_bold(self) -> bool
    {
        self.foreground_color < 16 && (self.foreground_color & 0b1000) != 0
    }

    pub fn set_bold(&mut self, is_bold: bool)
    {
        if self.foreground_color < 16  {
            if is_bold {
                self.foreground_color |= 0b0000_1000;
            } else {
                self.foreground_color &= 0b1111_0111;
            }
        }
    }

    pub fn is_blink(self) -> bool
    {
        if self.background_color < 16 { 
            self.background_color & 0b1000_0000 != 0
        } else {
            false
        }
    }

    pub fn set_blink(&mut self, is_blink: bool)
    {
        if self.background_color < 16 { 
            if is_blink {
                self.background_color |= 0b1000;
            } else {
                self.background_color &= 0b0111;
            }
        }
    }

    pub fn get_foreground(self) -> u8
    {
        self.foreground_color
    }

    pub fn get_foreground_without_bold(self) -> u8
    {
        self.foreground_color & 0b0000_0111
    }

    pub fn set_foreground(&mut self, color: u8) 
    {
        self.foreground_color = color;
    }

    pub fn set_foreground_without_bold(&mut self, color: u8) 
    {
        assert!(color < 0b1000);
        if self.foreground_color < 16  {
            self.foreground_color = (0b1000 & self.foreground_color) | color;
        }
    }

    pub fn get_background(self) -> u8
    {
        self.background_color
    }

    pub fn set_background(&mut self, color: u8) 
    {
        assert!(color < 0b1000, "color was: {}", color);
        self.background_color = color;
    }
}

impl PartialEq for TextAttribute {
    fn eq(&self, other: &TextAttribute) -> bool {
        self.foreground_color == other.foreground_color && self.background_color == other.background_color
    }
}
