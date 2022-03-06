use std::{rc::Rc, cell::RefCell};

use crate::{model::{TheDrawFont, Size, Rectangle, TextAttribute}, WORKSPACE};

use super::{Tool, MKey, Event, Editor, MKeyCode, MModifiers, Position};
use walkdir::{DirEntry, WalkDir};
pub struct FontTool {
    pub selected_font: i32,
    pub fonts: Vec<TheDrawFont>,
    pub sizes: Vec<Size>
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
    fn use_selection(&self) -> bool { false }

    fn handle_click(&mut self, editor: Rc<RefCell<Editor>>, button: u32, pos: Position) -> Event {
        if button == 1 {
            editor.borrow_mut().cursor.set_position(pos);
        }
        let mut editor = editor.borrow_mut();
        self.sizes.clear();
        editor.cur_selection = None;
        Event::None
    }

    fn handle_key(&mut self, editor: Rc<RefCell<Editor>>, key: MKey, _key_code: MKeyCode, modifier: MModifiers) -> Event
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
                editor.set_cursor(0,pos.y + font.get_font_height() as i32);
/* 
                if let Some(size) = self.sizes.last() {
                    editor.set_cursor(0,pos.y + size.height as i32);
                } else {
                    editor.set_cursor(0,pos.y + 1);
                }*/
                self.sizes.clear();
            }

            MKey::Backspace => {
                let letter_size= self.sizes.pop().unwrap_or_else(|| Size::from(1,1));
                editor.cur_selection = None;
                let pos = editor.cursor.get_position();
                if pos.x > 0 {
                    editor.cursor.set_position(pos + Position::from(-(letter_size.width as i32), 0));
                    if editor.cursor.insert_mode {
                        for i in pos.x..(editor.buf.width as i32 - (letter_size.width as i32)) {
                            let next = editor.get_char_from_cur_layer( Position::from(i + (letter_size.width as i32), pos.y));
                            editor.set_char(Position::from(i, pos.y), next);
                        }
                        let last_pos = Position::from(editor.buf.width as i32 - (letter_size.width as i32), pos.y);
                        editor.fill(Rectangle{ start: last_pos, size: letter_size }, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    } else {
                        let pos = editor.cursor.get_position();
                        editor.fill(Rectangle{ start: pos, size: letter_size }, super::DosChar { char_code: b' ', attribute: TextAttribute::DEFAULT });
                    } 
                }
            }

            MKey::Character(ch) => { 
                let c_pos = editor.cursor.get_position();
                editor.begin_atomic_undo();
                let attr = editor.cursor.get_attribute();
                let opt_size = font.render(&mut editor, c_pos, attr, ch);
                if let Some(size) = opt_size  {
                    editor.set_cursor(c_pos.x + size.width as i32 + font.spaces, c_pos.y);
                    let new_pos = editor.cursor.get_position();
                    self.sizes.push(Size { width: (new_pos.x - c_pos.x) as usize, height: size.height });
                } else {
                    editor.type_key(ch);
                    self.sizes.push(Size::from(1, 1));
                }
                editor.end_atomic_undo();
            }
            _ => {}
        }
        Event::None
    }
}