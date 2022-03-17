use std::{cell::RefCell, rc::Rc};

use gtk4::{traits::{BoxExt, CheckButtonExt, WidgetExt, StyleContextExt, ToggleButtonExt, OrientableExt}, CheckButton, ToggleButton, Orientation, Align };
use crate::{model::{RECT_TOOL, DrawMode}, ui::MainWindow};
fn set_char(char_code: u16)
{
    unsafe {
        RECT_TOOL.char_code = char_code;
    }
}
pub fn add_rectangle_tool_page(main_window: std::rc::Rc<MainWindow>, content_box: &mut gtk4::Box)
{
    unsafe {
        content_box.set_orientation(Orientation::Vertical);
        content_box.set_margin_top(20);
        content_box.set_margin_start(20);
        content_box.set_margin_bottom(20);
        content_box.set_spacing(20);
        
        let color_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .build();
        color_box.style_context().add_class("linked");

        let fg_button = ToggleButton::builder()
            .label("Fg")
            .active(RECT_TOOL.use_fore)
            .build();
        color_box.append(&fg_button);

        let bg_button = ToggleButton::builder()
            .label("Bg")
            .active(RECT_TOOL.use_back)
            .build();
        color_box.append(&bg_button);
        content_box.append(&color_box);

        let mode_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Start)
        .build();

        let line_checkbox = CheckButton::builder()
            .label("Line")
            .active(matches!(RECT_TOOL.draw_mode, DrawMode::Line))
            .build();
        mode_box.append(&line_checkbox);

        let char_container = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Start)
        .build();

        let char_checkbox = CheckButton::builder()
            .label("Character")
            .group(&line_checkbox)
            .active(matches!(RECT_TOOL.draw_mode, DrawMode::Char))
            .build();
        char_container.append(&char_checkbox);

        let button = Rc::new(RefCell::new(crate::ui::create_char_button(main_window.clone(), RECT_TOOL.char_code, Box::new(&set_char))));
        char_container.append(&button.borrow().button);
        main_window.char_buttons.borrow_mut().push(button);
        mode_box.append(&char_container);

        let shade_checkbox = CheckButton::builder()
            .label("Shade")
            .group(&line_checkbox)
            .active(matches!(RECT_TOOL.draw_mode, DrawMode::Shade))
            .build();
        mode_box.append(&shade_checkbox);

        let colorize_checkbox = CheckButton::builder()
            .label("Colorize")
            .group(&line_checkbox)
            .active(matches!(RECT_TOOL.draw_mode, DrawMode::Colorize))
            .build();
        mode_box.append(&colorize_checkbox);
        content_box.append(&mode_box);

        let fill_container = gtk4::Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .halign(Align::Start)
            .build();

        content_box.append(&fill_container);

        fg_button.connect_toggled(move |x| {
            RECT_TOOL.use_fore = x.is_active();
        });

        bg_button.connect_toggled(move |x| {
            RECT_TOOL.use_back = x.is_active();
        });

        line_checkbox.connect_toggled(|x| {
            if x.is_active() {
                RECT_TOOL.draw_mode = DrawMode::Line;
            }
        });

        char_checkbox.connect_toggled(|x| {
            if x.is_active() {
                RECT_TOOL.draw_mode = DrawMode::Char;
            }
        });

        shade_checkbox.connect_toggled(|x| {
            if x.is_active() {
                RECT_TOOL.draw_mode = DrawMode::Shade;
            }
        });

        colorize_checkbox.connect_toggled(|x| {
            if x.is_active() {
                RECT_TOOL.draw_mode = DrawMode::Colorize;
            }
        });
    }
}
