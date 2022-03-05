
#[derive(Clone, Copy, Debug, Default)]
pub struct TextAttribute(u8);

impl std::fmt::Display for TextAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Attr: {:X}, fg {}, bg {}, blink {})", self.as_u8(), self.get_foreground(), self.get_background(), self.is_blink())
    }
}

impl TextAttribute
{
    pub const DEFAULT : TextAttribute = TextAttribute(7);

    pub fn from_u8(attr: u8) -> Self
    {
        TextAttribute(attr)
    }

    pub fn from_color(fg: u8, bg: u8) -> Self
    {
        TextAttribute(fg | bg << 4)
    }

    pub fn as_u8(self) -> u8
    {
        self.0
    }

    pub fn is_bold(self) -> bool
    {
        self.0 & 0b0000_1000 != 0
    }

    pub fn set_bold(&mut self, is_bold: bool)
    {
        if is_bold {
            self.0 |= 0b0000_1000;
        } else {
            self.0 &= 0b1111_0111;
        }
    }

    pub fn is_blink(self) -> bool
    {
        self.0 & 0b1000_0000 != 0
    }

    pub fn set_blink(&mut self, is_blink: bool)
    {
        if is_blink {
            self.0 |= 0b1000_0000;
        } else {
            self.0 &= 0b0111_1111;
        }
    }

    pub fn get_foreground(self) -> u8
    {
        self.0 & 0b0000_1111
    }

    pub fn get_foreground_without_bold(self) -> u8
    {
        self.0 & 0b0000_0111
    }

    pub fn set_foreground(&mut self, color: u8) 
    {
        assert!(color < 0b1_0000);
        self.0 = (0b1111_0000 & self.0) | color;
    }

    pub fn set_foreground_without_bold(&mut self, color: u8) 
    {
        assert!(color < 0b1000);
        self.0 = (0b1111_1000 & self.0) | color;
    }

    pub fn get_background(self) -> u8
    {
        (self.0 >> 4) & 0b0111
    }

    pub fn get_background_ice(self) -> u8
    {
        self.0 >> 4
    }

    pub fn set_background(&mut self, color: u8) 
    {
        assert!(color < 0b1000, "color was: {}", color);
        self.0 = (0b1000_1111 & self.0) | (color << 4);
    }

    pub fn set_background_ice(&mut self, color: u8) 
    {
        assert!(color < 0b1_0000, "color was: {}", color);
        self.0 = (0b0000_1111 & self.0) | (color << 4);
    }
}

impl PartialEq for TextAttribute {
    fn eq(&self, other: &TextAttribute) -> bool {
        self.0 == other.0
    }
}
