use std::{ process::Command};

use glib::{clone, StaticType};
use gtk4::{traits::{BoxExt, WidgetExt, OrientableExt, ButtonExt}, Orientation, PropertyExpression, StringObject, pango::{AttrList, self}};
use crate::{model::{ FONT_TOOL}, ui::MainWindow, WORKSPACE};

pub fn add_font_tool_page(main_window: std::rc::Rc<MainWindow>, content_box: &mut gtk4::Box)
{
    unsafe {
        content_box.set_orientation(Orientation::Vertical);
        content_box.set_margin_top(20);
        content_box.set_margin_start(20);
        content_box.set_margin_end(20);
        content_box.set_margin_bottom(20);
        content_box.set_spacing(8);
        
        let title_label = gtk4::Label::new(Some("Ansi Font:"));
        title_label.set_wrap(true);
        title_label.set_wrap_mode(gtk4::pango::WrapMode::Char);
        content_box.append(&title_label);

        let list_model = gtk4::StringList::new(&[]);
        for i in 0..FONT_TOOL.fonts.len() {
            let font = &FONT_TOOL.fonts[i];
            list_model.append(&font.name);
        }

        if FONT_TOOL.fonts.is_empty() { 
            list_model.append("(none installed)");
        }
        // TODO: Filter with substring.
        let item_string_x = PropertyExpression::new(
            StringObject::static_type(), 
            gtk4::Expression::NONE, 
            "string");

        let font_dropdown = gtk4::DropDown::new(Some(&list_model), Some(item_string_x));
        font_dropdown.set_enable_search(true);
        font_dropdown.set_sensitive(!FONT_TOOL.fonts.is_empty());
        content_box.append(&font_dropdown);
        
        let str = String::from_utf8_unchecked((b'!'..=b'`').collect());
        let font_char_label = gtk4::Label::new(Some(str.as_str()));
        font_char_label.set_wrap(true);
        font_char_label.set_wrap_mode(gtk4::pango::WrapMode::Char);
        if !FONT_TOOL.fonts.is_empty() { 
            content_box.append(&font_char_label);
        }
    
        let open_font_manager_button = gtk4::Button::builder()
            .label("Font manager")
            .build();
        // TODO: Would maybe a nice feature to have a cool overview over all fonts
        // content_box.append(&open_font_manager_button);

        let open_path_button = gtk4::Button::builder()
            .label("Open font path…")
            .build();
        content_box.append(&open_path_button);
        font_dropdown.set_selected(0);
        crate::model::FONT_TOOL.selected_font = 0;
        update_available_chars(&font_char_label);

        font_dropdown.connect_selected_notify(move |x| {
            if FONT_TOOL.fonts.is_empty() { return; }
            let idx = x.selected() as i32;
            crate::model::FONT_TOOL.selected_font = idx;
            update_available_chars(&font_char_label);
        });

        open_font_manager_button.connect_clicked(clone!(@weak main_window => move |_| {
            crate::ui::font_manager_dialog::display_font_manager_dialog(main_window);
        }));

        open_path_button.connect_clicked(move |_| {
            if let Some(path) = &WORKSPACE.settings.font_path {
                // On Windows: "explorer" on macos: "open"
                Command::new("xdg-open")
                    .arg(path.as_os_str())
                    .spawn()
                    .unwrap();
            }
        });
    }
}

fn update_available_chars(label: &gtk4::Label)
{
    unsafe {
        if let Some(font) = FONT_TOOL.get_selected_font() {
            let attrs = AttrList::new();
            let mut old_index = 0;
            let mut has_font = false;
            for i in b'!'..=b'`' {
                let cur_has_font = font.has_char(i);
                if has_font == cur_has_font { continue; }
                let col = if has_font { 0x00} else { 0x7f00 };
                let mut attr = pango::AttrColor::new_foreground(col, col,col);
                attr.set_start_index(old_index);
                let new_index = (i - b'!') as u32;
                attr.set_end_index(new_index);
                attrs.insert(attr);
                
                has_font = cur_has_font;
                old_index = new_index;
            }

            let col = if has_font { 0x00} else { 0x7f00 };
            let mut attr = pango::AttrColor::new_foreground(col, col,col);
            attr.set_start_index(old_index as u32);
            attr.set_end_index(b'`' as u32 + 1);
            attrs.insert(attr);

            label.set_attributes(Some(&attrs));
        }
    }
}