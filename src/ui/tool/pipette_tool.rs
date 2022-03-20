use std::{
    cell::{Ref, RefCell},
    rc::Rc, str::FromStr,
};

use crate::{
    model::{BitFont, Size, DosChar},
    ui::MainWindow,
};
use gtk4::{
    gdk, prelude::{GdkCairoContextExt, DrawingAreaExtManual}, traits::{BoxExt, WidgetExt}, Orientation, Align,
};

#[allow(unused_must_use)]
pub fn add_pipette_tool_page(main_window: std::rc::Rc<MainWindow>, content_box: &mut gtk4::Box) {
    content_box.set_margin_top(20);
    content_box.set_margin_start(20);
    content_box.set_margin_bottom(20);

    
    let font_size = Size::<u8>::DEFAULT;
    let mut char_img = gtk4::cairo::ImageSurface::create(
        gtk4::cairo::Format::ARgb32,
        font_size.width as i32,
        font_size.height as i32,
    )
    .unwrap();
    
    let background_rgba = gdk::RGBA::from_str("black").unwrap();
    let main_window2 = RefCell::new(main_window);

    let disp_ch = Rc::new(RefCell::new(Some(DosChar::new())));
    let drawing_area = gtk4::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .valign(Align::Center)
        .halign(Align::Center)
        .vexpand(false)
        .build();
    let main_window3 = main_window2.clone();
    let disp_ch2 = disp_ch.clone();

    drawing_area
        .set_draw_func(move |_, cr, width, height| {
            let ch = (*disp_ch2.borrow()).unwrap_or_default();

            GdkCairoContextExt::set_source_rgba(cr, &background_rgba);
            cr.paint().expect("Invalid cairo surface state");
            {
                let mut data = char_img.data().expect("Can't lock image");
                let ptr = data.as_mut_ptr();
                if let Some(editor) = main_window3.borrow().get_current_editor() {
                    let fg = editor.borrow().buf.palette.colors[ch.attribute.get_foreground() as usize];
                    let bg = editor.borrow().buf.palette.colors[ch.attribute.get_background() as usize];

                    render_char(main_window3.borrow(), ch.char_code , ptr, fg.get_rgb(), bg.get_rgb());
                }
            }

            cr.translate(width as f64 / 2.0, height as f64 / 2.0);
            cr.scale(1.8, 1.8);
            cr.set_source_surface(
                &char_img,
                -char_img.width() as f64 / 2.0,
                -char_img.height() as f64 / 2.0,
            )
            .expect("error while calling fill.");

            cr.paint().expect("error while calling fill.");
        });

    let char_label = gtk4::Label::new(Some("Char ? (?)"));
    let color_label = gtk4::Label::new(Some("Fg ?, Bg ?"));
    let content_box2 = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    content_box2.append(&drawing_area);
    content_box2.append(&char_label);
    content_box2.append(&color_label);
    content_box.append(&content_box2);
    main_window2.borrow().pipette_update.replace(Box::new( move |c| {
        disp_ch.replace(c);

        if let Some(ch) = c {
            char_label.set_label(format!("Char {} (0x{0:04X})", ch.char_code).as_str());
            color_label.set_label(format!("Fg {}, Bg {}, Blink {}", ch.attribute.get_foreground(), ch.attribute.get_background(), ch.attribute.is_blink()).as_str());
        } else {
            char_label.set_label("");
            color_label.set_label("");
        }
        drawing_area.queue_draw();
    }));

}

fn render_char2(font: &BitFont, ch: u16, ptr: *mut u8, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
    let font_dimensions = font.size;
    let mut i = 0;
    unsafe {
        for y in 0..font_dimensions.height {
            let line = font.get_scanline(ch as u8, y as usize);
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
                    *ptr.add(i) = bg.2;
                    i += 1;
                    *ptr.add(i) = bg.1;
                    i += 1;
                    *ptr.add(i) = bg.0;
                    i += 1;
                    *ptr.add(i) = 255;
                    i += 1;
                }
            }
        }
    }
}

fn render_char(main_window: Ref<Rc<MainWindow>>, ch: u16, ptr: *mut u8, fg: (u8, u8, u8), bg: (u8, u8, u8)) {
    if let Some(editor) = main_window.get_current_editor() {
        render_char2(&editor.borrow().buf.font, ch, ptr, fg, bg);
    } else {
        render_char2(&BitFont::default(), ch, ptr, fg, bg);
    };
}
