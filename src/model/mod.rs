#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::too_many_lines, clippy::cast_lossless, clippy::cast_precision_loss)]

mod text_attribute;
use std::cmp::{min};

pub use text_attribute::*;

mod dos_char;
pub use dos_char::*;

mod layer;
pub use layer::*;

mod position;
pub use position::*;

mod buffer;
pub use  buffer::*;

mod formats;
pub use formats::*;

mod tdf_font;
pub use tdf_font::*;

mod sauce_handling;
pub use sauce_handling::*;

mod editor;
pub use editor::*;

pub(crate) mod tool;
pub use tool::*;

mod undo_stack;
pub use undo_stack::*;

mod palette_handling;
pub use palette_handling::*;

mod fonts;
pub use fonts::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Size<T> 
{
    pub width: T,
    pub height: T
}

impl<T> PartialEq for Size<T>
where T: PartialEq {
    fn eq(&self, other: &Size<T>) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<T> Size<T> 
where T: Default
{
    pub const DEFAULT:Size<u8> = Size { width:8, height: 16 };
    
    pub fn new() -> Self
    {
        Size::from(T::default(), T::default())
    }

    pub fn from(width: T, height: T) -> Self
    {
        Size { width, height }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle
{
    pub start: Position,
    pub size: Size<i32>
}

impl Rectangle 
{
    pub fn from(x: i32, y: i32, width: i32, height: i32) -> Self
    {
        Rectangle {
            start: Position::from(x,y), 
            size: Size::from(width, height) 
        }
    }

    pub fn from_coords(x1: i32, y1: i32, x2: i32, y2: i32) -> Self
    {
        assert!(x1 <= x2);
        assert!(y1 <= y2);
        Rectangle {
            start: Position::from(x1,y1), 
            size: Size::from((x2 - x1) + 1, (y2 - y1) + 1) 
        }
    }

    pub fn from_pt(p1: Position, p2: Position) -> Self
    {
        let start =Position::from(min(p1.x, p2.x), min(p1.y, p2.y));

        Rectangle {
            start, 
            size: Size::from((p1.x - p2.x).abs() + 1, (p1.y - p2.y).abs() + 1) 
        }
    }

    pub fn is_inside(&self, p: Position) -> bool
    {
        self.start.x <= p.x && 
        self.start.y <= p.y && 
        p.x < self.start.x + self.size.width as i32 &&
        p.y < self.start.y + self.size.height as i32
    }
}