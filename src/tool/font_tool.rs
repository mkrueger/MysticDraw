use std::path::{Path};

use gtk4::{traits::BoxExt, gdk::{Key, ModifierType}};
use crate::{font::TheDrawFont, editor::{EditorEvent, Editor}, model::Position};

use super::Tool;

pub struct FontTool {
    pub fonts: Vec<TheDrawFont>
}

impl FontTool 
{
    pub fn load_fonts(&mut self)
    {
        // self.fonts.push(TheDrawFont::load(Path::new("/home/mkrueger/Dokumente/THEDRAWFONTS/1911.TDF")).unwrap());
    }
}

impl Tool for FontTool
{
    fn get_icon_name(&self) -> &'static str { "edit-select" }

    fn add_tool_page(&self, parent: &mut gtk4::Box)
    {
        parent.append(&gtk4::Label::builder().label("FontTool").build());
    }
    
    fn handle_click(&self, editor: &mut Editor, _button: u32, x: i32, y: i32) -> EditorEvent
    {
        editor.cursor.pos = Position::from(x, y);
        EditorEvent::None
    }

    fn handle_key(&self, editor: &mut Editor, key: Key, _key_code: u32, _modifier: ModifierType) -> EditorEvent
    {
        match key {
            Key::Down => {
                editor.set_cursor(editor.cursor.pos.x, editor.cursor.pos.y + 1);
            }
            Key::Up => {
                editor.set_cursor(editor.cursor.pos.x, editor.cursor.pos.y - 1);
            }
            Key::Left => {
                editor.set_cursor(editor.cursor.pos.x - 1, editor.cursor.pos.y);
            }
            Key::Right => {
                editor.set_cursor(editor.cursor.pos.x + 1, editor.cursor.pos.y);
            }
            
            Key::Page_Down => {
                // TODO
            }
            
            Key::Page_Up => {
                // TODO
            }
            
            Key::Home | Key::KP_Home => {
                editor.set_cursor(0, editor.cursor.pos.y);
            }
            
            Key::End | Key::KP_End => {
                editor.set_cursor(editor.buf.width as i32 - 1, editor.cursor.pos.y);
            }

            Key::Return | Key::KP_Enter => {
                editor.set_cursor(0,editor.cursor.pos.y + self.fonts[0].get_font_height() as i32);
            }

            _ => { 
                if let Some(key) = key.to_unicode() {
                    
                    if key.len_utf8() == 1 {
                        let mut dst = [0];
                        key.encode_utf8(&mut dst);

                        let width =self.fonts[0].render(&mut editor.buf, editor.cursor.pos, editor.cursor.attr, dst[0]);
                        if width > 0 {
                            editor.set_cursor(editor.cursor.pos.x + width + self.fonts[0].spaces, editor.cursor.pos.y);
                        } else {
                            editor.buf.set_char(editor.cursor.pos, crate::model::DosChar {
                                char_code: dst[0],
                                attribute: editor.cursor.attr,
                            });
                            editor.set_cursor(editor.cursor.pos.x + 1, editor.cursor.pos.y);
                        }
                    }
                }
            }
        }
        EditorEvent::None
    }

}