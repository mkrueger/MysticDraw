use glib::{clone, Cast};
use gtk4::{traits::{BoxExt, ListBoxRowExt}};
use libadwaita::ApplicationWindow;

mod model;
pub use model::*;
mod model_imp;

mod row_data;
pub use row_data::*;
mod row_data_imp;

mod list_box_row;
pub use list_box_row::*;
mod list_box_row_imp;

pub fn add_font_tool_page(window: &ApplicationWindow, parent: &mut gtk4::Box)
{
    let model = FontModel::new();

    let listbox = gtk4::ListBox::new();
    listbox.bind_model(
        Some(&model), // 
        clone!(@weak window => @default-panic, move |item| {
            FontListBoxRow::new(
                item.downcast_ref::<FontRowData>()
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
            model.append(&FontRowData::new(&font.name.to_string(), i as u32));
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
