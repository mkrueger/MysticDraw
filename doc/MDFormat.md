# Mystic Draw formats

When designing ANSI tools there are several formats over the years that serve different purposes. For such an old technology all formats should've been already invented.
However this is not the case - unfortunately. I never wanted to add a format on the pile of character format variations - I swear.

But I had to… 

## Features

It's not ment as a distribution format like all others. This is an edit format just for ANSI drawing tools.
The main reason for inventing that was the support for multiple layers and I wanted to fix the file format problem for my tool. 
All formats have the issue that they're not lossless.

- ANSI - old and surely one of the best. It's possible to even preserve r,g,b values - but you're losing the font, palette order (in case of RGB)
- PCB, Avatar, BIN - only 16 colors with fixed palette, no font
- Artworx/Ice/XBIN - 16 colors
- Tundra - no palette order, no font (why?)

And not a single one has layers. Transparency is no feature of any of these format but can be simulated with goto xy.
Time for something new later generations of tool developers will maybe need to implement :).

## Goals

- Every supported format should be represented. Including tundra.
  - Roundtrip should be lossless
- Be compatible to Sauce/XBin models as much as possible.
- Try to be extensible
  - Don't waste too much space (contradicts a bit the extensibility)

## Format

The basic structure of the file is:
```
[ID]       3      'MDf'
[EOF]      1      EOF Char, usually 0x1A hex
[Checksum] 4      BE_U32 CRC32 checksum for [HEADER] and [BLOCKS]
[HEADER]   83     HEADER
[BLOCKS]   *      BLOCKS
<EOF>
```

### Header

```
Field      Bytes  Meaning
[VER]      2      BE_U16 u8 Major:u8 Minor - [00:00] atm
[Title]   35      CP 437 Chars - filled with b' ' SAUCE string
[Author]  20      CP 437 Chars - filled with b' ' SAUCE string
[Group]   20      CP 437 Chars - filled with b' ' SAUCE string
[Width]    2      BE_U16
[Height]   2      BE_U16
[Flags]    2      BE_U16 [Bit 1: iCE] [Bit 2: Save sauce] [Bit 3: 512 Char mode]
```

I esp. love the checksum part. A binary format is not ment to be altered with hex editors.

### Blocks

Until EOF - read blocks.

#### Comment block (only 1 is valid)

Basically the SAUCE comments due to sauce compability only one is valid.

```
Field      Bytes  Meaning
[1]        1      ID == 1
[NUM]      1      number of comments (max 255 - 0 is wasted)
[1]..[n]   n*64   Comment line CP 437 0 Terminated - 64 chars max
```

#### Palette block (only 1 is valid - open for discussion)

Probably too huge, but tundra basically means full palette is possible. Can 256^3 can easily be supported.
```
Field      Bytes  Meaning
[2]        1      ID == 2
[NUM]      4      BE_I32 number of colors (atm only 0xFFFF colors are supported - but it may change)
                  In future (maybe): -1 means no numbers and RGB values are directly stored in the Layer    
[1]..[n]   n*3    U8 ,g,b values from 0..255
```

#### Font Name

When using a standard font just store the name of the font. TODO: Get a list of standard font names.

```
Field      Bytes  Meaning
[3]        1      ID == 3
[Name]     22     Font name CP 437 0 Terminated - 22 chars max
```

#### Monochromatic font block

Font is a bit more flexible than in the other formats - however due to sauce the 'font name' is always an option.
So there is no real limit here. Extended fonts just get splitted into 2 256 font blocks.

```
Field      Bytes  Meaning
[4]        1      ID == 4
[Name]     22     Font name CP 437 0 Terminated - 22 chars max
[Width]    1      U8 - 1..32 width  - all other values are invalid
[Height]   1      U8 - 1..32 height - all other values are invalid
[Flags]    1      U8 Unused 
[Data]     *      Height * 256 * Byte Width - Note: Stored as BE
```

Note either ID = 2 or ID = 3 is valid. If no font data is availabe fallback to 8x16 DOS.

#### Layer

Layer order is important first one is the highest layer lowest layer is the last one (background)
Due to the support of 'dead' char/attribute pairs I needed a solution for that. Basically I chose XBIN as compression format - I really like it.

```
Field      Bytes  Meaning
[5]        1      ID == 5
[Title_Len]2      U16 length of the utf8 title
[Title]    *      U8 - UTF8 encoded chars - Note: May only be 16 chars depending on language.
[Mode]     1      U8 - Unused yet
[Flags]    2      BE_U16
                  [Bit 1   : Compression on/off]
                  [Bit 2-3 : Attribute Length 00 - U8, 01 - 2xU8 (FORE/BACK), - 10 - 2xU16(FORE/BACK), - 11 2xU32(FORE/BACK)]
                  [Bit 4   : Char Length - 0 - U8 1 - U16]
                  [Bit 5   : Unused]
                  [Bit 6   : is_visible]
                  [Bit 7   : edit_locked]
                  [Bit 8   : position_locked]
[X]        4      BE_I32
[Y]        4      BE_I32
[Width]    2      BE_U16  - No need for storing the "height" the data determines that. No idea why all formats store that info
[Data]     *      Data blocks, skip if width == 0
```

Data block:

```
Field      Bytes  Meaning
[Len]      2      BE_U16 n = 15 Bit - length, BIT 16 == 1 - append n empty chars, 00 == DONE
[Data]     *      Uncompressed:
                  [CHAR] : 1/2 Depending on flags
                  [ATTR] : U8, 2xU8, 3xU8, 4xU8 Depending on flags
                  Compression - See XBin
```