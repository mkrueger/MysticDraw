pub const DEFAULT_ATTRIBUTE: TextAttribute = TextAttribute(7);

#[derive(Clone, Copy, Debug, Default)]
pub struct TextAttribute(u8);

impl TextAttribute
{
    pub fn from_u8(attr: u8) -> Self
    {
        TextAttribute(attr)
    }

    pub fn as_u8(&self) -> u8
    {
        self.0
    }

    pub fn is_bold(&self) -> bool
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

    pub fn is_blink(&self) -> bool
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

    pub fn get_foreground(&self) -> u8
    {
        self.0 & 0b0000_1111
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

    pub fn get_background(&self) -> u8
    {
        self.0 >> 4
    }

    pub fn set_background(&mut self, color: u8) 
    {
        assert!(color < 0b1000, "color was: {}", color);
        self.0 = (0b1000_1111 & self.0) | (color << 4);
    }
}