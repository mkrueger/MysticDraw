#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap, clippy::too_many_lines, clippy::cast_lossless, clippy::cast_precision_loss)]

mod text_attribute;
use std::cmp::{min};

pub use text_attribute::*;

mod dos_char;
pub use dos_char::*;

mod layer;
pub use layer::*;

mod overlay_layer;
pub use overlay_layer::*;

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


#[derive(Copy, Clone, Debug, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

impl Size 
{
    pub fn new() -> Self
    {
        Size::from(0, 0)
    }

    pub fn from(width: usize, height: usize) -> Self
    {
        Size { width, height }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle
{
    pub start: Position,
    pub size: Size
}

impl Rectangle 
{
    pub fn from(x: i32, y: i32, width: usize, height: usize) -> Self
    {
        Rectangle {
            start: Position::from(x,y), 
            size: Size::from(width, height) 
        }
    }

    pub fn from_pt(p1: Position, p2: Position) -> Self
    {
        let start =Position::from(min(p1.x, p2.x), min(p1.y, p2.y));

        Rectangle {
            start, 
            size: Size::from((p1.x - p2.x).abs() as usize, (p1.y - p2.y).abs() as usize) 
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