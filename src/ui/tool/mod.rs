use glib::{clone, Cast};
use gtk4::{traits::{BoxExt, ListBoxRowExt}};
use libadwaita::ApplicationWindow;

mod font_tool;
pub use font_tool::*;

mod fill_tool;
pub use fill_tool::*;

mod draw_rectangle_tool;
pub use draw_rectangle_tool::*;

mod draw_ellipse_tool;
pub use draw_ellipse_tool::*;

mod draw_line_tool;
pub use draw_line_tool::*;

pub fn add_font_tool_page(window: &ApplicationWindow, parent: &mut gtk4::Box)
{
    let model = font_tool::Model::new();

    let listbox = gtk4::ListBox::new();
    listbox.bind_model(
        Some(&model), // 
        clone!(@weak window => @default-panic, move |item| {
            font_tool::ListBoxRow::new(
                item.downcast_ref::<font_tool::RowData>()
                    .expect("RowData is of wrong type"),
            )
            .upcast::<gtk4::Widget>()
        }),
    );

    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(480)
        .min_content_width(360)
        .build();

    scrolled_window.set_child(Some(&listbox));
    
    unsafe {
        for i in 0..crate::model::FONT_TOOL.fonts.len() {
            let font = &crate::model::FONT_TOOL.fonts[i];
            model.append(&RowData::new(&font.name.to_string(), i as u32));
        }
    }

    listbox.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let idx = row.index();
            unsafe {
                crate::model::FONT_TOOL.selected_font = idx;
            }
        }   
    });


    
    parent.append(&scrolled_window);
}
