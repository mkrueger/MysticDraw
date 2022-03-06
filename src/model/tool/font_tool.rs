use std::{rc::Rc, cell::RefCell};

use crate::{model::TheDrawFont, WORKSPACE};

use super::{Tool, MKey, Event, Editor, MKeyCode, MModifiers};
use walkdir::{DirEntry, WalkDir};
pub struct FontTool {
    pub selected_font: i32,
    pub fonts: Vec<TheDrawFont>,
    pub last_height: i32
}

impl FontTool 
{
    fn is_hidden(entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map_or(false, |s| s.starts_with('.'))
    }
        
    pub fn load_fonts(&mut self)
    {
        if let Some(path) = unsafe { &WORKSPACE.settings.font_path } {
            let walker = WalkDir::new(path).into_iter();
            for entry in walker.filter_entry(|e| !FontTool::is_hidden(e)) {
                if let Err(e) = entry {
                    eprintln!("Can't load tdf font library: {}", e);
                    break;
                }
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    continue;
                }
                let extension = path.extension();
                if extension.is_none() { continue; }
                let extension = extension.unwrap().to_str();
                if extension.is_none() { continue; }
                let extension = extension.unwrap().to_lowercase();

                if extension == "tdf" {
                    if let Some(font) = TheDrawFont::load(path) {
                        self.fonts.push(font);
                    }
                }
            }
        }
        println!("{} fonts read.", self.fonts.len());
    }
}

impl Tool for FontTool
{
    fn get_icon_name(&self) -> &'static str { "md-tool-font" }

    fn handle_key(&mut self, editor: Rc<RefCell<Editor>>, key: MKey, key_code: MKeyCode, modifier: MModifiers) -> Event
    {
        if self.selected_font < 0 || self.selected_font >= self.fonts.len() as i32 {
            return Event::None;
        }
        let font = &self.fonts[self.selected_font as usize];
        let pos = editor.borrow().cursor.get_position();
        let mut editor = editor.borrow_mut();

        match key {
            MKey::Down => {
                editor.set_cursor(pos.x, pos.y + 1);
            }
            MKey::Up => {
                editor.set_cursor(pos.x, pos.y - 1);
            }
            MKey::Left => {
                editor.set_cursor(pos.x - 1, pos.y);
            }
            MKey::Right => {
                editor.set_cursor(pos.x + 1, pos.y);
            }
            
            MKey::Home  => {
                if let MModifiers::Control = modifier {
                    for i in 0..editor.buf.width {
                        if !editor.get_char_from_cur_layer(pos.with_x(i as i32)).is_transparent() {
                            editor.set_cursor(i as i32, pos.y);
                            return Event::None;
                        }
                    }
                }
                editor.set_cursor(0, pos.y);
            }

            MKey::End => {
                if let MModifiers::Control = modifier {
                    for i in (0..editor.buf.width).rev()  {
                        if !editor.get_char_from_cur_layer(pos.with_x(i as i32)).is_transparent() {
                            editor.set_cursor(i as i32, pos.y);
                            return Event::None;
                        }
                    }
                }
                let w = editor.buf.width as i32;
                editor.set_cursor(w - 1, pos.y);
            }

            MKey::Return => {
                editor.set_cursor(0,pos.y + self.last_height);
            }

            MKey::Character(ch) => { 
                let cpos = editor.cursor.get_position();
                let attr = editor.cursor.get_attribute();
                let optSize = font.render(&mut editor, cpos, attr, ch);
                if let Some(size) = optSize  {
                    editor.set_cursor(cpos.x + size.width as i32 + font.spaces, cpos.y);
                    self.last_height = size.height as i32;
                } else {
                    editor.type_key(ch);
                    self.last_height = 1;
                }
            }
            _ => {}
        }
        Event::None
    }
}