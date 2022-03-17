use std::{rc::Rc, cell::RefCell};

use gtk4::{traits::{BoxExt, CheckButtonExt, WidgetExt, StyleContextExt, ToggleButtonExt, OrientableExt}, CheckButton, ToggleButton, Orientation, Align, Label, SpinButton};
use crate::{model::{BRUSH_TOOL, brush_imp::BrushType}, ui::MainWindow};


fn set_char(char_code: u16)
{
    unsafe {
        BRUSH_TOOL.char_code = char_code;
    }
}

pub fn add_brush_tool_page(main_window: std::rc::Rc<MainWindow>, content_box: &mut gtk4::Box)
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
            .active(BRUSH_TOOL.use_fore)
            .build();
        color_box.append(&fg_button);

        let bg_button = ToggleButton::builder()
            .label("Bg")
            .active(BRUSH_TOOL.use_back)
            .build();
        color_box.append(&bg_button);
        content_box.append(&color_box);

        let size_container = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .halign(Align::Start)
        .build();

        let size_label = Label::builder()
            .label("Size")
            .build();
        size_container.append(&size_label);

        let size_button = SpinButton::with_range(0.0, 64.0, 1.0);
        size_button.set_value(3.0);
        size_container.append(&size_button);
        content_box.append(&size_container);

        let mode_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Start)
        .build();

        let shade_checkbox = CheckButton::builder()
            .label("Shade")
            .active(matches!(BRUSH_TOOL.brush_type, BrushType::Shade))
            .build();
        mode_box.append(&shade_checkbox);

        let char_container = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Start)
        .build();

        let char_checkbox = CheckButton::builder()
            .label("Character")
            .group(&shade_checkbox)
            .active(matches!(BRUSH_TOOL.brush_type, BrushType::Solid))
            .build();
        char_container.append(&char_checkbox);

        let button = Rc::new(RefCell::new(crate::ui::create_char_button(main_window.clone(), BRUSH_TOOL.char_code, Box::new(&set_char))));
        char_container.append(&button.borrow().button);
        mode_box.append(&char_container);
        
        main_window.char_buttons.borrow_mut().push(button);

        let colorize_checkbox = CheckButton::builder()
            .label("Colorize")
            .group(&shade_checkbox)
            .active(matches!(BRUSH_TOOL.brush_type, BrushType::Color))
            .build();
            mode_box.append(&colorize_checkbox);
        content_box.append(&mode_box);

        fg_button.connect_toggled(move |x| {
            BRUSH_TOOL.use_fore = x.is_active();
        });

        bg_button.connect_toggled(move |x| {
            BRUSH_TOOL.use_back = x.is_active();
        });

        shade_checkbox.connect_toggled(|x| {
            if x.is_active() {
                BRUSH_TOOL.brush_type = BrushType::Shade;
            }
        });

        char_checkbox.connect_toggled(|x| {
            if x.is_active() {
                BRUSH_TOOL.brush_type = BrushType::Solid;
            }
        });

        colorize_checkbox.connect_toggled(|x| {
            if x.is_active() {
                BRUSH_TOOL.brush_type = BrushType::Color;
            }
        });

        size_button.connect_value_changed(|b| {
            BRUSH_TOOL.size = b.value_as_int();
        });
    }
}
