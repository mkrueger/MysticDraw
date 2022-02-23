use std::path::PathBuf;

use crate::sauce::Sauce;

use super::Layer;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct BitFont {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u16>,
}

#[derive(Debug)]
pub struct Buffer {
    pub file_name: Option<PathBuf>,

    pub width: usize,
    pub height: usize,

    pub base_layer: Layer,
    pub font: Option<BitFont>,
    pub layers: Vec<Layer>,
    pub sauce: Option<Sauce>,
}
impl Buffer {
    pub fn new() -> Self {
        Buffer {
            file_name: None,
            width: 0,
            height: 0,
            base_layer: Layer::new(),
            font: None,
            layers: Vec::new(),
            sauce: None,
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer::new()
    }
}
