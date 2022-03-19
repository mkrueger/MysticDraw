use std::{
    cell::RefCell,
    rc::Rc,
};

use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::{
    gdk::{self, Key, ModifierType, Rectangle},
    glib,
    traits::{GestureDragExt, GestureExt, GestureSingleExt, WidgetExt, PopoverExt},
};

use crate::{
    model::{Editor, MKey, MKeyCode, MModifiers, Position},
};

use self::gtkansi_view::GtkAnsiView;
mod gtkansi_view;
use crate::model::TOOLS;
use crate::WORKSPACE;

glib::wrapper! {
    pub struct AnsiView(ObjectSubclass<GtkAnsiView>) @extends gtk4::Widget, gtk4::DrawingArea;
}

impl Default for AnsiView {
    fn default() -> Self {
        Self::new()
    }
}

static mut DRAG_POS: Position = Position {x:-1, y:-1};

impl AnsiView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn set_mimap_mode(&self, is_minimap: bool)
    {
        self.imp().set_mimap_mode(is_minimap);
    } 

    pub fn set_preview_rectangle(&self, rect: Option<crate::model::Rectangle>)
    {
        self.imp().set_preview_rectangle(rect);
        self.queue_draw();
    } 

    pub fn get_is_mimap(&self) -> bool
    {
        *self.imp().is_minimap.borrow()
    } 

    pub fn get_editor(&self) -> Rc<RefCell<Editor>> {
        self.imp().editor.borrow().clone()
    }

    fn calc_xy(&self, c: &Rc<RefCell<Editor>>, xy: (f64, f64)) -> Position {

        let (sx, sy) = self.imp().get_start_pos(self);

        let dim = c.borrow().buf.get_font_dimensions();
        let x = xy.0 - sx as f64;
        let y = xy.1 - sy as f64;
        Position::from(
            (x / dim.width as f64) as i32,
            (y / dim.height as f64) as i32,
        )
    }

    fn translate_key(key: Key) -> Option<MKey> {
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
                        return Some(MKey::Character(dst[0] as u16));
                    }
                }
                None
            }
        }
    }

    fn translate_modifier(modifier: gdk::ModifierType) -> MModifiers {
        match modifier {
            ModifierType::SHIFT_MASK => MModifiers::Shift,
            ModifierType::ALT_MASK => MModifiers::Alt,
            ModifierType::CONTROL_MASK => MModifiers::Control,
            _ => MModifiers::None,
        }
    }
    fn translate_key_code(key_code: u32) -> MKeyCode {
        match key_code {
            29 => MKeyCode::KeyY,
            30 => MKeyCode::KeyU,
            31 => MKeyCode::KeyI,
            _ => MKeyCode::Unknown,
        }
    }
    pub fn set_editor_handle(&self, handle: Rc<RefCell<Editor>>) {
        let buffer = &handle.borrow().buf;
        let font_dimensions = buffer.get_font_dimensions();

        self.imp().set_editor_handle(handle.clone());
        if !self.get_is_mimap() {
            self.set_size_request(
                buffer.width as i32 * font_dimensions.width as i32,
                buffer.height as i32 * font_dimensions.height as i32,
            );
        } else {
            return;
        }

        // let dialog = Dialog { payload: editor };
        if !handle.borrow().is_inactive {
            let drag = gtk4::GestureDrag::new();
            let handle1 = handle.clone();
            drag.connect_begin(glib::clone!(@strong self as this => move |gst_drag, _| {
                let start = gst_drag.start_point();
                let cur   = gst_drag.offset();
                if start.is_none() || cur.is_none() {
                    return;
                }
                let start = this.calc_xy(&handle1, start.unwrap());
                let end   = this.calc_xy(&handle1, cur.unwrap());
                unsafe {
                    DRAG_POS = start;
                    TOOLS[WORKSPACE.selected_tool].handle_drag_begin(handle1.clone(), start, end);
                }
                this.queue_draw();
                this.grab_focus();
            }));

            let handle1 = handle.clone();
            drag.connect_end(glib::clone!(@strong self as this => move |gst_drag, _| {
                let start = gst_drag.start_point();
                let cur   = gst_drag.offset();
                if start.is_none() || cur.is_none() {
                    return;
                }
                let start = start.unwrap();
                let cur = cur.unwrap();
                let cur = (start.0 + cur.0, start.1 + cur.1);
                let start = this.calc_xy(&handle1, start);
                let end   = this.calc_xy(&handle1, cur);
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_drag_end(handle1.clone(), start, end);
                }
                this.queue_draw();
                this.grab_focus();
            }));

            let handle1 = handle.clone();
            drag.connect_update(glib::clone!(@strong self as this => move |gst_drag, _| {
                let start = gst_drag.start_point();
                let cur   = gst_drag.offset();
                if start.is_none() || cur.is_none() {
                    return;
                }
                let start = start.unwrap();
                let cur = cur.unwrap();
                let cur = (start.0 + cur.0, start.1 + cur.1);
                let start = this.calc_xy(&handle1, start);
                let end   = this.calc_xy(&handle1, cur);
                unsafe {
                    if DRAG_POS != end {
                        DRAG_POS = end;
                        TOOLS[WORKSPACE.selected_tool].handle_drag(handle1.clone(), start, end);
                        this.queue_draw();
                    }
                }
                this.grab_focus();
            }));
            self.add_controller(&drag);

            let gesture = gtk4::GestureClick::new();
            let handle1 = handle.clone();
            gesture.set_button(1);
            gesture.connect_pressed(glib::clone!(@strong self as this => move |e, _clicks, x, y| {
                let pos = this.calc_xy(&handle1, (x, y));
                unsafe {
                    TOOLS[WORKSPACE.selected_tool].handle_click(handle1.clone(), e.button(), pos);
                }
                this.queue_draw();
                this.grab_focus();
            }));
            self.add_controller(&gesture);

            let gesture = gtk4::GestureClick::new();
            gesture.set_button(3);

            let menu_model = gtk4::gio::Menu::new();
            menu_model.append(Some("Cut"), Some("app.cut"));
            menu_model.append(Some("Copy"), Some("app.copy"));
            menu_model.append(Some("Paste"), Some("app.paste"));
            menu_model.append(Some("Erase"), Some("app.erase"));
            
            menu_model.append(Some("Left justify"), Some("app.left_justify"));
            menu_model.append(Some("Center"), Some("app.center_justify"));
            menu_model.append(Some("Right justify"), Some("app.right_justify"));

            menu_model.append(Some("Rotate"), Some("app.right_justify"));
            menu_model.append(Some("Flip X"), Some("app.flip_x"));
            menu_model.append(Some("Flip Y"), Some("app.flip_y"));
            menu_model.append(Some("Crop"), Some("app.crop"));

            let menu = gtk4::PopoverMenu::from_model(Some(&menu_model));
            menu.set_parent(self);

            gesture.connect_pressed(glib::clone!(@strong self as this => move |_, _clicks, x, y| {
                menu.set_pointing_to(Some(&Rectangle::new(x as i32, y as i32, 1, 1)));
                menu.popup();
            }));
            self.add_controller(&gesture);

            let handle1 = handle.clone();
            let key = gtk4::EventControllerKey::new();
            key.connect_key_pressed(glib::clone!(@strong self as this => move |_, key, key_code, modifier| {
                {
                    if let Some(key)= AnsiView::translate_key(key) {
                        unsafe {
                            TOOLS[WORKSPACE.selected_tool].handle_key(handle1.clone(), key, AnsiView::translate_key_code(key_code), AnsiView::translate_modifier(modifier));
                        }
                        this.queue_draw();
                    }
                }
                glib::signal::Inhibit(true)
            }));
            self.add_controller(&key);
            self.queue_draw();
        }
    }
}
