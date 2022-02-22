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
    pub file_name: Box<PathBuf>,
    pub base_layer: Layer,
    pub font: Option<BitFont>,
    pub layers: Vec<Layer>,
    pub sauce: Option<Sauce>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            file_name: Box::new(PathBuf::new()),
            base_layer: Layer::new(),
            font: None,
            layers: Vec::new(),
            sauce: None,
        }
    }
}
