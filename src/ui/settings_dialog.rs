use std::{rc::Rc, str::FromStr, cell::RefCell};

use glib::{ObjectExt, SignalHandlerId, StaticType};
use gtk4::{ traits::{ WidgetExt, BoxExt, GtkWindowExt, ButtonExt, FlowBoxChildExt, EventControllerExt, GestureSingleExt, GridExt, DrawingAreaExt }, SpinButton, Orientation, Align, gdk, prelude::{DrawingAreaExtManual, GdkCairoContextExt}, SelectionMode, PropertyExpression, StringObject };
use libadwaita::{ PreferencesGroup, ActionRow, traits::{PreferencesGroupExt, ActionRowExt}, HeaderBar, ViewSwitcherBar };

use crate::{WORKSPACE, model::{TheDrawFont, BitFont, Buffer, Position, TextAttribute, DosChar, Editor, Rectangle}};

use super::{MainWindow, AnsiView};

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
        .name("PreferencesWindow")
        .width_request(480)
        .height_request(440)
        .modal(true)
        .resizable(false)
        .content(&main_area)
        .build();
    dialog.set_transient_for(Some(&main_window.window));

    let switcher_title = libadwaita::ViewSwitcherTitle::builder()
        .name("switcher_title")
        .build();


    let hb = HeaderBar::builder()
        .title_widget(&switcher_title)
        .show_end_title_buttons(true)
        .build();
    let open_button = gtk4::Button::builder()
        .label("OK")
        .build();
    hb.pack_start(&open_button);
    main_area.append(&hb);

    let stack = libadwaita::ViewStack::builder()
        .vexpand(true)
        .build();
    main_area.append(&stack);

    let switch_bar = ViewSwitcherBar::builder()
        .name("switcher_bar")
        .build();
    switch_bar.set_stack(Some(&stack));

    dialog.bind_property("title", &switcher_title, "title")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();
    switcher_title.bind_property("title-visible", &switch_bar, "reveal")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();

    main_area.append(&switch_bar);
    
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
    stack.add_titled(&content_area, Some("General"), "General");

    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

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
    stack.add_titled(&content_area, Some("TDF_Font"), "TDF Font");
    stack.add_titled(&generated_function_key_page(), Some("FunctionKeys"), "Function Keys");

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

fn generated_function_key_page() -> gtk4::Box {
    let content_area = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(20)
        .margin_top(20)
        .margin_end(20)
        .margin_start(20)
        .spacing(8)
        .build();

    let selector_area = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(Align::Center)
        .halign(Align::Start)
        .spacing(8)
        .build();

    let list_model = gtk4::StringList::new(&[]);
    list_model.append("(default)");
    let item_string_x = PropertyExpression::new(
        StringObject::static_type(), 
        gtk4::Expression::NONE, 
        "string");

    let font_dropdown = gtk4::DropDown::new(Some(&list_model), Some(item_string_x));

    let insert_button = gtk4::Button::builder()
        .label("Insert")
        .build();
    let delete_button = gtk4::Button::builder()
        .label("Delete")
        .sensitive(false)
        .build();

    selector_area.append(&gtk4::Label::builder()
        .label("Function keys for font:")
        .valign(Align::Center)
        .build());

        selector_area.append(&font_dropdown);
    selector_area.append(&insert_button);
    selector_area.append(&delete_button);

    content_area.append(&selector_area);


    let grid = gtk4::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let mut editors = Vec::new();
    for i in 0..15 {
        let label = gtk4::Label::builder()
            .label(format!("Set {}", i + 1).as_str())
            .valign(Align::Center)
            .halign(Align::End)
            .build();
        let mut buffer = Buffer::new();
            buffer.font = BitFont::default();
            buffer.width = 12;
            buffer.height = 1;
        
        for j in 0..10 {
            buffer.set_char(0, Position::from(j,  0), Some(DosChar::from_u8_char(crate::model::DEFAULT_OUTLINE_TABLE[i][j as usize])));
        }

        let editor = Editor::new(0, buffer);
        let editor_handle = Rc::new(RefCell::new(editor));
        let charset_view = AnsiView::new();
        charset_view.set_valign(Align::Center);
        charset_view.set_width_request(12 * 16);
        charset_view.set_height_request(8);
        
        charset_view.set_valign(gtk4::Align::Center);
        charset_view.set_editor_handle(editor_handle);

        let x = i as i32;
        grid.attach(&label, (x / 5) * 2, x % 5, 1, 1);
        grid.attach(&charset_view, (x / 5) * 2 + 1, x % 5, 1, 1);
        editors.push(Rc::new(RefCell::new(charset_view)));
    }
    content_area.append(&grid);

    let char_label = gtk4::Label::new(None);

    let char_table = create_char_table(&BitFont::default(), 0, editors, &char_label);

    let char_area = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();
    char_area.append(&char_table);

    content_area.append(&char_area);
    content_area.append(&char_label);

    content_area
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
            let line = font.get_scanline(ch as u8, y as usize);
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

const CHARS_PER_LINE : u16 = 32;
fn create_char_table(font: &BitFont, font_number: u16, editors: Vec<Rc<RefCell<AnsiView>>>, char_label: &gtk4::Label) -> AnsiView
{
    let mut buffer = Buffer::new();
    buffer.font = font.clone();
    buffer.width = CHARS_PER_LINE;
    buffer.height = 256 / CHARS_PER_LINE;

    for y in 0..buffer.height {
        for x in 0..buffer.width {
            buffer.set_char(0, Position::from(x as i32, y as i32), Some(DosChar {
                char_code: (y * CHARS_PER_LINE + x) as u16,
                attribute: TextAttribute::DEFAULT
            }));
        }
    }

    let mut editor = Editor::new(0, buffer);
    editor.is_inactive = true;
    let key_handle = Rc::new(RefCell::new(editor));

    let charset_view = AnsiView::new();
    charset_view.set_mimap_mode(true);
    charset_view.set_can_focus(false);
    charset_view.set_width_request((CHARS_PER_LINE * font.size.width as u16 * 2) as i32);
    charset_view.set_height_request((256 / CHARS_PER_LINE * font.size.height as u16 * 2) as i32);
    charset_view.set_editor_handle(key_handle);
    let gesture = gtk4::EventControllerMotion::new();

    let font_width  = font.size.width as u16;
    let font_height = font.size.height as u16;

    gesture.connect_leave(glib::clone!(@strong charset_view as this => move |_| {
        this.set_preview_rectangle(None);
    }));

    gesture.connect_motion(glib::clone!(@strong charset_view as this, @weak char_label => move |_, x, y| {
        let x = (x / 2.0) as u16;
        let y = (y / 2.0) as u16;

        let my_char = x / font_width + CHARS_PER_LINE * (y / font_height);
        set_selected_char(&this, &char_label, my_char);
    }));
    charset_view.add_controller(&gesture);

    let gesture = gtk4::GestureClick::new();
    gesture.set_button(1);

    let font_width  = font.size.width as u16;
    let font_height = font.size.height as u16;

    gesture.connect_pressed(glib::clone!(@strong charset_view as this, @weak char_label, @strong editors => move |_, _clicks, x, y| {
        let x = (x / 2.0) as u16;
        let y = (y / 2.0) as u16;

        let my_char = x / font_width + CHARS_PER_LINE * (y / font_height);

        for e in &editors { 
            let editor = e.borrow();
            if editor.has_focus() {
                editor.get_editor().borrow_mut().type_key(my_char | (font_number << 8));
                editor.queue_draw();
                break;
            }
        }
    
        this.queue_draw();
        this.grab_focus();
    }));
    charset_view.add_controller(&gesture);

    charset_view
}

fn set_selected_char(view: &AnsiView, label: &gtk4::Label, char_code: u16)
{
    view.set_preview_rectangle(Some(
        Rectangle::from(
            (char_code % CHARS_PER_LINE) as i32, 
            (char_code / CHARS_PER_LINE) as i32, 
            1, 
            1
        )
    ));
    label.set_text(format!("Char: {}, (0x{0:>02X})", char_code).as_str());
}
