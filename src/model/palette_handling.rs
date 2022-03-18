#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color{r, g, b }
    }
    pub fn get_rgb_f64(self) -> (f64, f64, f64) {
        (
            self.r as f64 / 255_f64,
            self.g as f64 / 255_f64,
            self.b as f64 / 255_f64
        )

    }
    
    pub fn get_rgb_f32(self) -> (f32, f32, f32) {
        (
            self.r as f32 / 255_f32,
            self.g as f32 / 255_f32,
            self.b as f32 / 255_f32
        )
    }
    
    pub fn get_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}
impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}


#[derive(Debug, Clone)]
pub struct Palette {
    pub colors: Vec<Color>
}

static EGA_COLOR_OFFSETS: [usize;16] = [ 0, 1, 2, 3, 4, 5, 20, 7, 56, 57, 58, 59, 60, 61, 62, 63 ];

pub const DOS_DEFAULT_PALETTE: [Color; 16] = [
    Color { r: 0x00, g: 0x00, b: 0x00 }, // black
    Color { r: 0x00, g: 0x00, b: 0xAA }, // blue
    Color { r: 0x00, g: 0xAA, b: 0x00 }, // green
    Color { r: 0x00, g: 0xAA, b: 0xAA }, // cyan
    Color { r: 0xAA, g: 0x00, b: 0x00 }, // red
    Color { r: 0xAA, g: 0x00, b: 0xAA }, // magenta
    Color { r: 0xAA, g: 0x55, b: 0x00 }, // brown
    Color { r: 0xAA, g: 0xAA, b: 0xAA }, // lightgray
    Color { r: 0x55, g: 0x55, b: 0x55 }, // darkgray
    Color { r: 0x55, g: 0x55, b: 0xFF }, // lightblue
    Color { r: 0x55, g: 0xFF, b: 0x55 }, // lightgreen
    Color { r: 0x55, g: 0xFF, b: 0xFF }, // lightcyan
    Color { r: 0xFF, g: 0x55, b: 0x55 }, // lightred
    Color { r: 0xFF, g: 0x55, b: 0xFF }, // lightmagenta
    Color { r: 0xFF, g: 0xFF, b: 0x55 }, // yellow
    Color { r: 0xFF, g: 0xFF, b: 0xFF }, // white
];

pub const EGA_PALETTE: [Color; 64] = [
	Color { r: 0x00, g: 0x00, b: 0x00 },
	Color { r: 0x00, g: 0x00, b: 0xAA },
	Color { r: 0x00, g: 0xAA, b: 0x00 },
	Color { r: 0x00, g: 0xAA, b: 0xAA },
	Color { r: 0xAA, g: 0x00, b: 0x00 },
	Color { r: 0xAA, g: 0x00, b: 0xAA },
	Color { r: 0xAA, g: 0xAA, b: 0x00 },
	Color { r: 0xAA, g: 0xAA, b: 0xAA },
	Color { r: 0x00, g: 0x00, b: 0x55 },
	Color { r: 0x00, g: 0x00, b: 0xFF },
	Color { r: 0x00, g: 0xAA, b: 0x55 },
	Color { r: 0x00, g: 0xAA, b: 0xFF },
	Color { r: 0xAA, g: 0x00, b: 0x55 },
	Color { r: 0xAA, g: 0x00, b: 0xFF },
	Color { r: 0xAA, g: 0xAA, b: 0x55 },
	Color { r: 0xAA, g: 0xAA, b: 0xFF },
	Color { r: 0x00, g: 0x55, b: 0x00 },
	Color { r: 0x00, g: 0x55, b: 0xAA },
	Color { r: 0x00, g: 0xFF, b: 0x00 },
	Color { r: 0x00, g: 0xFF, b: 0xAA },
	Color { r: 0xAA, g: 0x55, b: 0x00 },
	Color { r: 0xAA, g: 0x55, b: 0xAA },
	Color { r: 0xAA, g: 0xFF, b: 0x00 },
	Color { r: 0xAA, g: 0xFF, b: 0xAA },
	Color { r: 0x00, g: 0x55, b: 0x55 },
	Color { r: 0x00, g: 0x55, b: 0xFF },
	Color { r: 0x00, g: 0xFF, b: 0x55 },
	Color { r: 0x00, g: 0xFF, b: 0xFF },
	Color { r: 0xAA, g: 0x55, b: 0x55 },
	Color { r: 0xAA, g: 0x55, b: 0xFF },
	Color { r: 0xAA, g: 0xFF, b: 0x55 },
	Color { r: 0xAA, g: 0xFF, b: 0xFF },
	Color { r: 0x55, g: 0x00, b: 0x00 },
	Color { r: 0x55, g: 0x00, b: 0xAA },
	Color { r: 0x55, g: 0xAA, b: 0x00 },
	Color { r: 0x55, g: 0xAA, b: 0xAA },
	Color { r: 0xFF, g: 0x00, b: 0x00 },
	Color { r: 0xFF, g: 0x00, b: 0xAA },
	Color { r: 0xFF, g: 0xAA, b: 0x00 },
	Color { r: 0xFF, g: 0xAA, b: 0xAA },
	Color { r: 0x55, g: 0x00, b: 0x55 },
	Color { r: 0x55, g: 0x00, b: 0xFF },
	Color { r: 0x55, g: 0xAA, b: 0x55 },
	Color { r: 0x55, g: 0xAA, b: 0xFF },
	Color { r: 0xFF, g: 0x00, b: 0x55 },
	Color { r: 0xFF, g: 0x00, b: 0xFF },
	Color { r: 0xFF, g: 0xAA, b: 0x55 },
	Color { r: 0xFF, g: 0xAA, b: 0xFF },
	Color { r: 0x55, g: 0x55, b: 0x00 },
	Color { r: 0x55, g: 0x55, b: 0xAA },
	Color { r: 0x55, g: 0xFF, b: 0x00 },
	Color { r: 0x55, g: 0xFF, b: 0xAA },
	Color { r: 0xFF, g: 0x55, b: 0x00 },
	Color { r: 0xFF, g: 0x55, b: 0xAA },
	Color { r: 0xFF, g: 0xFF, b: 0x00 },
	Color { r: 0xFF, g: 0xFF, b: 0xAA },
	Color { r: 0x55, g: 0x55, b: 0x55 },
	Color { r: 0x55, g: 0x55, b: 0xFF },
	Color { r: 0x55, g: 0xFF, b: 0x55 },
	Color { r: 0x55, g: 0xFF, b: 0xFF },
	Color { r: 0xFF, g: 0x55, b: 0x55 },
	Color { r: 0xFF, g: 0x55, b: 0xFF },
	Color { r: 0xFF, g: 0xFF, b: 0x55 },
	Color { r: 0xFF, g: 0xFF, b: 0xFF },
];

impl Palette {
    pub fn new() -> Self {
        Palette { colors: DOS_DEFAULT_PALETTE.to_vec() }
    }

    pub fn len(&self) -> u32 {
        self.colors.len() as u32
    }
    
    pub fn clear(&mut self) {
        self.colors.clear();
    }
    
    pub fn fill_to_16(&mut self) {
        if self.colors.len() < DOS_DEFAULT_PALETTE.len()  {
            self.colors.extend(&DOS_DEFAULT_PALETTE[self.colors.len()..]);
        }
    }

    pub fn is_default(&self) -> bool {
        if self.colors.len() != DOS_DEFAULT_PALETTE.len() { return false; }
        #[allow(clippy::needless_range_loop)]
        for i in 0..DOS_DEFAULT_PALETTE.len() {
            if self.colors[i] != DOS_DEFAULT_PALETTE[i] { return false; }
        }
        true
    }

    pub fn get_color(&mut self, r: u8, g: u8, b: u8) -> u8 {

        for i in 0..self.colors.len() {
            let col = self.colors[i];
            if col.r == r && col.g == g && col.b == b { return i as u8; }
        }
        self.colors.push(Color { r, g, b});
        (self.colors.len() - 1) as u8
    }

    pub fn from(pal: &[u8]) -> Self {
        let mut colors = Vec::new();
        let mut o = 0;
        while o < pal.len() {
            colors.push(Color {
                r: pal[o] << 2 | pal[o] >> 4,
                g: pal[o + 1] << 2 | pal[o + 1] >> 4,
                b: pal[o + 2] << 2 | pal[o + 2] >> 4
            });
            o += 3;
        }

        Palette { colors }
    }
    
    pub fn cycle_ega_colors(&self) -> Palette {
        let mut colors = self.colors.clone();
        #[allow(clippy::needless_range_loop)]
        for i in 0..EGA_COLOR_OFFSETS.len() {
            let offset = EGA_COLOR_OFFSETS[i];
            if i == offset { continue; }
            colors.swap(i, offset);
        }
        Palette { colors }
    }

    pub fn to_ega_palette(&self) -> Vec<u8>
    {
        let mut ega_colors ;
        
        if self.colors.len() == 64 {
            //assume ega palette
            ega_colors = self.colors.clone();
            #[allow(clippy::needless_range_loop)]
            for i in 0..EGA_COLOR_OFFSETS.len() {
                let offset = EGA_COLOR_OFFSETS[i];
                if i == offset { continue; }
                ega_colors.swap(i, offset);
            }
        } else {
            // just store the first 16 colors to the standard EGA palette
            ega_colors = EGA_PALETTE.to_vec();
            for i in 0..16 {
                if i >= self.colors.len() { break; }
                ega_colors[EGA_COLOR_OFFSETS[i]] = self.colors[i];
            }
        }
        let mut res = Vec::with_capacity(3 * 64);
        for col in ega_colors {
            res.push(col.r >> 2 | col.r << 4);
            res.push(col.g >> 2 | col.g << 4);
            res.push(col.b >> 2 | col.b << 4);
        }
        res
    }

    pub fn to_16color_vec(&self) -> Vec<u8>
    {
        let mut res = Vec::with_capacity(3 * 16);
        #[allow(clippy::needless_range_loop)]
        for i in 0..16 {
            let col = if i < self.colors.len()  { self.colors[i] }  else { DOS_DEFAULT_PALETTE[i] };

            res.push(col.r >> 2 | col.r << 4);
            res.push(col.g >> 2 | col.g << 4);
            res.push(col.b >> 2 | col.b << 4);
        }
        res
    }

    pub fn to_vec(&self) -> Vec<u8>
    {
        let mut res = Vec::new();
        res.resize(3 * self.colors.len(), 0);
        for col in &self.colors {
            res.push(col.r >> 2 | col.r << 4);
            res.push(col.g >> 2 | col.g << 4);
            res.push(col.b >> 2 | col.b << 4);
        }
        res
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}