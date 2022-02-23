use glium::{
    implement_vertex,  Frame, Surface,
    
};
use std::cell::Cell;
use std::rc::Rc;

use crate::model::Position;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);
pub struct CharRenderer {
    context: Rc<glium::backend::Context>,
    pub editor_id: Cell<usize>,
    textures: Vec<glium::Texture2d>,

    width: Cell<i32>,
    height: Cell<i32>
}

impl CharRenderer {
    pub fn new(buffer_id: usize, context: Rc<glium::backend::Context>) -> Self {
        let mut character_textures = Vec::new();

        for color in 0..16 {
            for ch in 0..=255 {
                let mut texture= Vec::new();
                for y in 0..16 {
                    let line = crate::DEFAULT_FONT[ch * 16 + y];// + (extendedFontMode && (attr & 8) == 8 ? 256 * 16 : 0));
                    for x in 0..8 {
                        if (line & (128 >> x)) != 0 {
                            texture.push(crate::model::DOS_DEFAULT_PALETTE[color].0);
                            texture.push(crate::model::DOS_DEFAULT_PALETTE[color].1);
                            texture.push(crate::model::DOS_DEFAULT_PALETTE[color].2);
                        } else {
                            texture.push(0_u8);
                            texture.push(0_u8);
                            texture.push(0_u8);
                        }
                    }
                }

                let image = glium::texture::RawImage2d::from_raw_rgb_reversed(texture.as_slice(), (8, 16));
                let opengl_texture = glium::Texture2d::new(&context, image).unwrap();
                character_textures.push(opengl_texture);
            }
        }

        CharRenderer {
            context,
            editor_id: Cell::new(buffer_id),
            textures: character_textures,
            width: Cell::new(0),
            height: Cell::new(0)
        }
    }

    pub fn iresize(&self, width: i32, height: i32)
    {
        self.width.set(width);
        self.height.set(height);
    }

    pub fn draw(&self) {
        let mut frame = Frame::new(
            self.context.clone(),
            (10 * 8, 10 * 16)
        );
        frame.clear_color(0., 0., 0., 1.);
        let editor = crate::Workspace::get_editor(self.editor_id.get());
        let buffer = &editor.buf;
        for y in 0..buffer.height {
            for x in 0..buffer.width {
                let ch  = buffer.get_char(&Position::from(x as i32, y as i32));
/* 
                frame.clear(Some(&Rect {
                    left: (x * 8) as u32,
                    bottom: (self.height.get() as usize - 16 - (y * 16)) as u32,
                    width: 8,
                    height: 16,
                }), Some(ch.get_background_srgb()), true,  None, None);

                */
/*
                let color = ch.attribute >> 4;

                let surface = self.textures[color as usize * 256 + 219 as usize].as_surface();
                surface.blit_whole_color_to(
                    &frame, 
                    &glium::BlitTarget {
                        left: (x * 8) as u32,
                        bottom: (self.height.get() as usize - 16 - (y * 16)) as u32,
                        width: 8,
                        height: 16,
                    }, 
                    glium::uniforms::MagnifySamplerFilter::Linear);
*/
                if self.height.get() - 16 - (y as i32 * 16) < 0  {
                    break;
                }
                if ch.char_code > 0 {
                    let color = ch.attribute.get_foreground();
                    let surface = self.textures[color as usize * 256 + ch.char_code as usize].as_surface();
                    surface.blit_whole_color_to(
                        &frame, 
                        &glium::BlitTarget {
                            left: (x * 8) as u32,
                            bottom: (self.height.get() as usize - 16 - (y * 16)) as u32,
                            width: 8,
                            height: 16,
                        }, 
                        glium::uniforms::MagnifySamplerFilter::Linear);
                }
            }

        }

        let surface = self.textures[15_usize * 256 + b'_' as usize].as_surface();
        surface.blit_whole_color_to(
            &frame, 
            &glium::BlitTarget {
                left: (editor.cursor.pos.x * 8) as u32,
                bottom: (self.height.get() as i32 - 16 - (editor.cursor.pos.y * 16)) as u32,
                width: 8,
                height: 16,
            }, 
            glium::uniforms::MagnifySamplerFilter::Linear);

        frame.finish().unwrap();
    }
}