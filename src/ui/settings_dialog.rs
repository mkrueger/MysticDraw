use std::{rc::Rc, str::FromStr};

use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt, ButtonExt, FlowBoxChildExt }, SpinButton, Orientation, Align, gdk, prelude::{DrawingAreaExtManual, GdkCairoContextExt}, SelectionMode };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}, HeaderBar };

use crate::{WORKSPACE, model::{TheDrawFont, BitFont}};

use super::MainWindow;

const OUTLINE_WIDTH: usize = 8;
const OUTLINE_HEIGHT: usize = 6;
const OUTLINE_FONT_CHAR: [u8; 48]= [
    69,65,65,65,65,65,65,70,
    67,79,71,66,66,72,79,68,
    67,79,73,65,65,74,79,68,
    67,79,71,66,66,72,79,68,
    67,79,68,64,64,67,79,68,
    75,66,76,64,64,75,66,76
];

pub struct SettingsDialog {
    pub dialog: libadwaita::PreferencesWindow,
    pub open_button: gtk4::Button,
    pub guide_dropdown: gtk4::DropDown,
    pub grid_dropdown: gtk4::DropDown,
    pub outline_box: gtk4::FlowBox
}

impl SettingsDialog {
    pub fn store_settings(&self)
    {
        unsafe {
            WORKSPACE.grid = std::mem::transmute(self.grid_dropdown.selected());
            WORKSPACE.guide = std::mem::transmute(self.guide_dropdown.selected());

            if let Some(child) = self.outline_box.selected_children().first() {
                WORKSPACE.settings.outline_font_style = child.index() as usize;
            }
        }
    }
}

pub fn display_settings_dialog(main_window: Rc<MainWindow>)
{
    let main_area = gtk4::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    let dialog = libadwaita::PreferencesWindow::builder()
        .width_request(480)
        .height_request(440)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
    dialog.set_transient_for(Some(&main_window.window));

    let hb = HeaderBar::builder()
        .title_widget(&libadwaita::WindowTitle::builder().title("Preferences").build())
        .show_end_title_buttons(true)
        .build();
    let open_button = gtk4::Button::builder()
        .label("OK")
        .build();
    hb.pack_start(&open_button);
    main_area.append(&hb);

    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

        let group = PreferencesGroup::new();
        group.set_title("Settings");
        
    let tab_size_spin_button = SpinButton::with_range(0.0, 10000.0, 10.0);
    unsafe {
        tab_size_spin_button.set_value(WORKSPACE.settings.tab_size as f64);
    }

    let row = ActionRow::builder()
        .title("Tab size")
        .build();
    row.add_suffix(&tab_size_spin_button);
    group.add(&row);

    let grid_names = [
        "Off",
        "4x2",
        "6x3",
        "8x4",
        "12x6",
        "16x8"
    ];
    
    let grid_dropdown = gtk4::DropDown::from_strings(&grid_names);
    grid_dropdown.set_valign(Align::Center);
    unsafe {
        grid_dropdown.set_selected(WORKSPACE.grid as u32);
    }

    let row = ActionRow::builder()
        .title("Show grid")
        .build();
    row.add_suffix(&grid_dropdown);
    group.add(&row);

    let guide_names = [
        "Off",
        "80x25",
        "80x40",
        "80x50",
        "44x22",
    ];
    
    let guide_dropdown = gtk4::DropDown::from_strings(&guide_names);
    guide_dropdown.set_valign(Align::Center);
    unsafe {
        guide_dropdown.set_selected(WORKSPACE.guide as u32);
    }

    let row = ActionRow::builder()
        .title("Show guide")
        .build();
    row.add_suffix(&guide_dropdown);
    group.add(&row);
    content_area.append(&group);

    let outline_box = gtk4::FlowBox::builder()
        .valign(Align::Start)
        .max_children_per_line(7)
        .min_children_per_line(6)
        .selection_mode(SelectionMode::Single)
        .build();
    
    for o in 0..TheDrawFont::OUTLINE_STYLES { //
        outline_box.append(&create_outline_button(o));
    }
    
    unsafe {
        if let Some(child) = outline_box.child_at_index(WORKSPACE.settings.outline_font_style as i32) {
            outline_box.select_child(&child);
        }
    }  

    content_area.append(&gtk4::Label::builder()
        .label("Outline font style")
        .halign(Align::Start)
        .build()
    );

    content_area.append(&outline_box);

    main_area.append(&content_area);

    dialog.show();
    let dialog = Rc::new(SettingsDialog {
        dialog,
        open_button,
        grid_dropdown,
        guide_dropdown,
        outline_box
    });

    dialog.clone().open_button.connect_clicked(move |_| {
        dialog.dialog.close();
        dialog.store_settings();
        main_window.update_layer_view();
        main_window.update_editor();
    });
}

fn create_outline_button(
    outline_style: usize,
) -> gtk4::DrawingArea {
    let drawing_area = gtk4::DrawingArea::builder()
        .content_width(OUTLINE_WIDTH as i32 * 8)
        .content_height(OUTLINE_HEIGHT as i32 * 16)
        .halign(Align::Center)
        .build();
    
    let mut char_img = gtk4::cairo::ImageSurface::create(
        gtk4::cairo::Format::ARgb32,
        8 * OUTLINE_WIDTH as i32,
        16 * OUTLINE_HEIGHT as i32,
    )
    .unwrap();
    let background_rgba = gdk::RGBA::from_str("black").unwrap();
    let default_font = BitFont::default();

    drawing_area.set_draw_func(move |_, cr, width, height| {
        GdkCairoContextExt::set_source_rgba(cr, &background_rgba);
        for y in 0..OUTLINE_HEIGHT {
            for x in 0..OUTLINE_WIDTH {
                let ch = TheDrawFont::transform_outline(outline_style, OUTLINE_FONT_CHAR[y * 8 + x]) as u16;
                {
                    let mut data = char_img.data().expect("Can't lock image");
                    let ptr = data.as_mut_ptr();

                    render_char2(&default_font, x, y, ch, ptr, (175, 175, 175));
                }
            }
        }
        
        cr.scale(width as f64 / char_img.width() as f64, height as f64 / char_img.height() as f64);
        cr.set_source_surface( &char_img, 0.0, 0.0).expect("error while calling fill.");
        cr.paint().expect("error while calling fill.");
    });
    drawing_area
}

fn render_char2(font: &BitFont, char_x: usize, char_y: usize, ch: u16, ptr: *mut u8, fg: (u8, u8, u8)) {
    let w = font.size.width as usize;
    let h = font.size.height as usize;
    let screen_x = char_x * w;
    let screen_y = char_y * h;
    unsafe {
        for y in 0..h {
            let line = font.get_scanline(ch, y as usize);
            for x in 0..w {
                let i = (screen_x + x) * 4 + (screen_y + y) * OUTLINE_WIDTH * w * 4;
                if (line & (128 >> x)) != 0 {
                    *ptr.add(i) = fg.2;
                    *ptr.add(i + 1) = fg.1;
                    *ptr.add(i + 2) = fg.0;
                    *ptr.add(i + 3) = 255;
                } else {
                    *ptr.add(i) = 0;
                    *ptr.add(i + 1) = 0;
                    *ptr.add(i + 2) = 0;
                    *ptr.add(i + 3) = 255;
                }
            }
        }
    }
}