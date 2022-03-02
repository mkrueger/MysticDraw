
use std::{ str::FromStr, rc::Rc, cell::RefCell, cmp::{max, min} };

use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::{ glib, traits::{WidgetExt, GestureExt, GestureSingleExt, GestureDragExt}, gdk::{Paintable, self, Key, ModifierType}, prelude::{DrawingAreaExtManual, GdkCairoContextExt}, cairo::Operator};

use crate::{model::{Position, Editor, Size, MKey, MModifiers, MKeyCode}, sync_workbench_state};

use self::gtkchar_editor_view::GtkCharEditorView;
mod gtkchar_editor_view;
use crate::model::TOOLS;
use crate::WORKSPACE;


glib::wrapper! {
    pub struct CharEditorView(ObjectSubclass<GtkCharEditorView>) @extends gtk4::Widget, gtk4::DrawingArea, @implements Paintable;
}

impl Default for CharEditorView {
    fn default() -> Self {
         Self::new()
    }
}

/*
struct Dialog {
    payload: Editor,
 }
*/


impl CharEditorView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn get_editor(&self) -> Rc<RefCell<Editor>>
    {
        self.imp().editor.borrow().clone()
    }

    fn calc_xy(c: &Rc<RefCell<Editor>>, xy: (f64, f64)) -> Position
    {
        let dim = c.borrow().buf.get_font_dimensions();
        let x = xy.0;
        let y = xy.1;
        Position::from((x / dim.width as f64) as i32, (y / dim.height as f64) as i32)
    }
    
    fn translate_key(key: Key) -> Option<MKey>
    {
        match key {
            Key::Down => Some(MKey::Down),
            Key::Up => Some(MKey::Up),
            Key::Left => Some(MKey::Left),
            Key::Right => Some(MKey::Right),
            Key::Page_Down => Some(MKey::PageDown),
            Key::Page_Up => Some(MKey::PageUp),

            Key::Home | Key::KP_Home => Some(MKey::Home),
            Key::End | Key::KP_End => Some(MKey::End),
            Key::Return | Key::KP_Enter => Some(MKey::Return),
            Key::Delete | Key::KP_Delete => Some(MKey::Delete),
            Key::Insert | Key::KP_Insert => Some(MKey::Insert),
            Key::BackSpace => Some(MKey::Backspace),
            Key::Tab => Some(MKey::Tab),
            Key::Escape => Some(MKey::Escape),


            Key::F1 => Some(MKey::F1),
            Key::F2 => Some(MKey::F2),
            Key::F3 => Some(MKey::F3),
            Key::F4 => Some(MKey::F4),
            Key::F5 => Some(MKey::F5),
            Key::F6 => Some(MKey::F6),
            Key::F7 => Some(MKey::F7),
            Key::F8 => Some(MKey::F8),
            Key::F9 => Some(MKey::F9),
            Key::F10 => Some(MKey::F10),
            Key::F11 => Some(MKey::F11),
            Key::F12 => Some(MKey::F12),

            _ => {
                if let Some(key) = key.to_unicode() {
                    if key.len_utf8() == 1 {
                        let mut dst = [0];
                        key.encode_utf8(&mut dst);
                        return Some(MKey::Character(dst[0]));
                    }
                }
                None
            }
        }
    }

    fn translate_modifier(modifier: gdk::ModifierType) -> MModifiers
    {
        match modifier {
            ModifierType::SHIFT_MASK => MModifiers::Shift,
            ModifierType::ALT_MASK => MModifiers::Alt,
            ModifierType::CONTROL_MASK => MModifiers::Control,
            _ => MModifiers::None
        }
    }
    fn translate_key_code(key_code: u32) -> MKeyCode
    {
        println!("key code: {}", key_code);
        match key_code {
            29 => MKeyCode::KeyY,
            30 => MKeyCode::KeyU,
            31 => MKeyCode::KeyI,
            _ => { MKeyCode::Unknown }
        }
        
    }
    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>)
    {
        let buffer = &handle.borrow().buf;
        let font_dimensions = buffer.get_font_dimensions();
        self.set_size_request((buffer.width * font_dimensions.width) as i32, (buffer.height * font_dimensions.height) as i32);
        self.imp().editor.replace(handle.clone());

        let mut char_img =
        gtk4::cairo::ImageSurface::create(gtk4::cairo::Format::ARgb32, 8, 16).unwrap();
       // let dialog = Dialog { payload: editor };
        if !handle.borrow().is_inactive {
            let drag = gtk4::GestureDrag::new();
            let handle1 = handle.clone();

            drag.connect_begin(glib::clone!(@strong self as this => move |gst_drag, _| {
                sync_workbench_state(&mut handle1.borrow_mut());
                let start = gst_drag.start_point();
                let cur   = gst_drag.offset();
                if start.is_none() || cur.is_none() {
                    return;
                }
                let start = CharEditorView::calc_xy(&handle1, start.unwrap());
                let end   = CharEditorView::calc_xy(&handle1, cur.unwrap());
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_drag_begin(handle1.clone(), start, end);
                }
                this.queue_draw();
                this.grab_focus();
            })); 

            let handle1 = handle.clone();
            drag.connect_end(glib::clone!(@strong self as this => move |gst_drag, _| {
                sync_workbench_state(&mut handle1.borrow_mut());
                let start = gst_drag.start_point();
                let cur   = gst_drag.offset();
                if start.is_none() || cur.is_none() {
                    return;
                }
                let start = CharEditorView::calc_xy(&handle1, start.unwrap());
                let end   = CharEditorView::calc_xy(&handle1, cur.unwrap());
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_drag_end(handle1.clone(), start, end);
                }
                this.queue_draw();
                this.grab_focus();
            })); 
            
            let handle1 = handle.clone();
            drag.connect_update(glib::clone!(@strong self as this => move |gst_drag, _| {
                sync_workbench_state(&mut handle1.borrow_mut());
                let start = gst_drag.start_point();
                let cur   = gst_drag.offset();
                if start.is_none() || cur.is_none() {
                    return;
                }
                let start = start.unwrap();
                let cur = cur.unwrap();
                let cur = (start.0 + cur.0, start.1 + cur.1);
                let start = CharEditorView::calc_xy(&handle1, start);
                let end   = CharEditorView::calc_xy(&handle1, cur);
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_drag(handle1.clone(), start, end);
                }
                this.queue_draw();
                this.grab_focus();
            }));
            self.add_controller(&drag);

            let gesture = gtk4::GestureClick::new();
            let handle1 = handle.clone();
            gesture.set_button(1);
            gesture.connect_pressed(glib::clone!(@strong self as this => move |e, _clicks, x, y| {
                sync_workbench_state(&mut handle1.borrow_mut());
                let x = min(handle1.borrow().buf.width as i32, max(0, x as i32 / font_dimensions.width as i32));
                let y = min(handle1.borrow().buf.height as i32, max(0, y as i32 / font_dimensions.height as i32));
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_click(handle1.clone(), e.button(),Position::from(x, y));
                }
                this.queue_draw();
                this.grab_focus();
            }));
            self.add_controller(&gesture);

            let gesture = gtk4::GestureClick::new();
            let handle1 = handle.clone();
            gesture.set_button(3);
            gesture.connect_pressed(glib::clone!(@strong self as this => move |e, _clicks, x, y| {
                sync_workbench_state(&mut handle1.borrow_mut());
                let x = min(handle1.borrow().buf.width as i32, max(0, x as i32 / font_dimensions.width as i32));
                let y = min(handle1.borrow().buf.height as i32, max(0, y as i32 / font_dimensions.height as i32));
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_click(handle1.clone(), e.button(), Position::from(x, y));
                }
                this.queue_draw();
                this.grab_focus();
            }));
            self.add_controller(&gesture);

            let handle1 = handle.clone();
            let key = gtk4::EventControllerKey::new();
            key.connect_key_pressed(glib::clone!(@strong self as this => move |_, key, key_code, modifier| {
                sync_workbench_state(&mut handle1.borrow_mut());
                {
                    if let Some(key)= CharEditorView::translate_key(key) {
                        unsafe {
                            TOOLS[WORKSPACE.selected_tool].handle_key(handle1.clone(), key, CharEditorView::translate_key_code(key_code), CharEditorView::translate_modifier(modifier));
                        }
                        this.queue_draw();
                    }
                }
                glib::signal::Inhibit(true)
            }));
            self.add_controller(&key);
        }

        let handle1 = handle.clone();
        let background_rgba = gdk::RGBA::from_str("white").unwrap();
        self.set_draw_func(move |_, cr, _width, _height| {
            GdkCairoContextExt::set_source_rgba(cr, &background_rgba);
            cr.paint().expect("Invalid cairo surface state");

            let editor = &handle1.borrow();
            let buffer = &editor.buf;

            let font_dimensions = buffer.get_font_dimensions();
    
            for y in 0..buffer.height {
                for x in 0..buffer.width {
                    let ch = buffer.get_char(Position::from(x as i32, y as i32));
                    
                    cr.rectangle(
                        x as f64 * font_dimensions.width as f64, 
                        y as f64 * font_dimensions.height as f64, 
                        font_dimensions.width as f64,
                        font_dimensions.height as f64);
                    let bg = buffer.get_rgb_f64(ch.attribute.get_background());
                    cr.set_source_rgba(bg.0, bg.1, bg.2, 1f64);
                    cr.fill().expect("error while calling fill.");
                    
                    let fg = buffer.get_rgb(ch.attribute.get_foreground());
                    unsafe {
                        let mut data = char_img.data().expect("Can't lock image");
                        let ptr = data.as_mut_ptr();
                        render_char(buffer, ch.char_code, ptr, fg);
                    }
                    cr.set_source_surface(&char_img, (x as i32 * font_dimensions.width as i32) as f64, (y as i32 * font_dimensions.height as i32) as f64)
                        .expect("error while calling fill.");
                        cr.paint()
                        .expect("error while calling fill.");
                }
            }
            if !editor.is_inactive {
                draw_caret(editor.cursor.get_position(), cr, font_dimensions);
    
                if editor.cur_selection.is_active {
                    let rect = &editor.cur_selection.rectangle;
                    cr.rectangle(
                        buffer.to_screenx(rect.start.x), 
                        buffer.to_screeny(rect.start.y), 
                        buffer.to_screenx(rect.size.width as i32), 
                        buffer.to_screeny(rect.size.height as i32));
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    cr.set_line_width(3f64);
                    println!("preview: {}", editor.cur_selection.is_preview);
                    if editor.cur_selection.is_preview {
                        cr.fill().expect("error while calling fill.");
                    } else {
                        cr.stroke_preserve().expect("error while calling stroke.");

                        cr.set_source_rgb(0.0, 0.0, 0.0);
                        cr.set_line_width(1f64);
                        cr.stroke().expect("error while calling stroke.");
                    }
                }
            }
        });
    }
}

fn draw_caret(cursor_pos: Position, cr: &gtk4::cairo::Context, font_dimensions: Size) {
    let x = cursor_pos.x;
    let y = cursor_pos.y;
    cr.rectangle((x as i32 * font_dimensions.width as i32) as f64, (y as i32 * font_dimensions.height as i32) as f64, font_dimensions.width as f64, font_dimensions.height as f64);
    cr.set_source_rgb(0x7F as f64 / 255.0, 0x7F as f64 / 255.0, 0x7F as f64 / 255.0);
    cr.set_operator(Operator::Difference);
    cr.fill().expect("error while calling fill.");
}

unsafe fn render_char(buffer: &crate::model::Buffer, ch: u8, ptr: *mut u8, fg: (u8, u8, u8)) {
    let font_dimensions = buffer.get_font_dimensions();
    let mut i = 0;
    for y in 0..font_dimensions.height {
        let line = buffer.get_font_scanline(ch, y as usize);
        for x in 0..font_dimensions.width {
            if (line & (128 >> x)) != 0 {
                *ptr.add(i) = fg.2;
                i += 1;
                *ptr.add(i) = fg.1;
                i += 1;
                *ptr.add(i) = fg.0;
                i += 1;
                *ptr.add(i) = 255;
                i += 1;
            } else {
                *ptr.add(i) = 0;
                i += 1;
                *ptr.add(i) = 0;
                i += 1;
                *ptr.add(i) = 0;
                i += 1;
                *ptr.add(i) = 0;
                i += 1;
            }
        }
    }
}