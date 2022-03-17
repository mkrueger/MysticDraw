use std::{
    cell::{Ref, RefCell},
    rc::Rc, str::FromStr,
};

use crate::{
    model::{BitFont, Size},
    ui::MainWindow,
};
use gtk4::{
    gdk, prelude::{GdkCairoContextExt, DrawingAreaExtManual}, traits::{BoxExt, WidgetExt}, Orientation,
};

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


    let ch = b'T' as u16;
    let drawing_area = gtk4::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .vexpand(false)
        .build();
    drawing_area
        .set_draw_func(move |_, cr, width, height| {
            GdkCairoContextExt::set_source_rgba(cr, &background_rgba);
            cr.paint().expect("Invalid cairo surface state");

            {
                let mut data = char_img.data().expect("Can't lock image");
                let ptr = data.as_mut_ptr();
                render_char(main_window2.borrow(), ch, ptr, (255, 255, 255));
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

    let label = gtk4::Label::new(Some("Char 100 (0x64)"));
    let content_box2 = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    content_box2.append(&drawing_area);
    content_box2.append(&label);

    content_box.append(&content_box2);

}

fn render_char2(font: &BitFont, ch: u16, ptr: *mut u8, fg: (u8, u8, u8)) {
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
}

fn render_char(main_window: Ref<Rc<MainWindow>>, ch: u16, ptr: *mut u8, fg: (u8, u8, u8)) {
    if let Some(editor) = main_window.get_current_editor() {
        render_char2(&editor.borrow().buf.font, ch, ptr, fg);
    } else {
        render_char2(&BitFont::default(), ch, ptr, fg);
    };
}
