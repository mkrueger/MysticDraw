use gtk4::{traits::{BoxExt, CheckButtonExt, WidgetExt, OrientableExt}, CheckButton, Orientation, Align, Label, SpinButton};
use crate::{model::{ERASE_TOOL, erase_imp::EraseType}};

pub fn add_erase_tool_page(content_box: &mut gtk4::Box)
{
    unsafe {
        content_box.set_orientation(Orientation::Vertical);
        content_box.set_margin_top(20);
        content_box.set_margin_start(20);
        content_box.set_margin_bottom(20);
        content_box.set_spacing(20);

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
            .active(matches!(ERASE_TOOL.brush_type, EraseType::Shade))
            .build();
        mode_box.append(&shade_checkbox);

        let solid_checkbox = CheckButton::builder()
            .label("Solid")
            .group(&shade_checkbox)
            .active(matches!(ERASE_TOOL.brush_type, EraseType::Solid))
            .build();
            mode_box.append(&solid_checkbox);
        content_box.append(&mode_box);

        shade_checkbox.connect_toggled(|x| {
            if x.is_active() {
                ERASE_TOOL.brush_type = EraseType::Shade;
            }
        });

        solid_checkbox.connect_toggled(|x| {
            if x.is_active() {
                ERASE_TOOL.brush_type = EraseType::Solid;
            }
        });

        size_button.connect_value_changed(|b| {
            println!("change value !!!");
            ERASE_TOOL.size = b.value_as_int();
        });
    }
}
