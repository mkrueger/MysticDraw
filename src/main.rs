use gtk4::{gio::{ApplicationFlags, self}};
use libadwaita as adw;

use adw::{prelude::*};
use gtk4::{Application};
use model::{init_tools, Editor, TextAttribute, Tool, TOOLS};
use ui::MainWindow;

mod model;
pub mod ui;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../data/font.fnt");

pub struct Workspace {
    selected_tool: usize,
    selected_attribute: TextAttribute,

    font_dimensions: model::Size
}

impl Workspace {
    pub fn cur_tool(&self) -> std::boxed::Box<&&'static mut dyn Tool> {
        unsafe {
            let t = &TOOLS[self.selected_tool];
            std::boxed::Box::new(t)
        }
    }
    pub fn get_font_dimensions(&self) -> model::Size { self.font_dimensions }
    pub fn get_font_scanline(&self, ch: u8, y: usize) -> u32
    {
        DEFAULT_FONT[ch as usize * 16 + y] as u32
    }
}

pub static mut WORKSPACE: Workspace = Workspace {
    selected_tool: 0,
    selected_attribute: TextAttribute::DEFAULT,
    font_dimensions: model::Size { width: 8, height: 16 }
};

pub fn sync_workbench_state(editor: &mut Editor) {
    // quite lame but unfortunately I don't see a sane way to really work
    // with the same state accross everything I'm not able to get any mutable data strucutures out of Gtk
    // and working with weird RefCell/Cell/Rc makes things worse than doing a manualy sync.
    unsafe {
        editor.cursor.set_attribute(WORKSPACE.selected_attribute);
    }
}

const RESOURCES_BYTES:&[u8] = include_bytes!("../data/resources.gresource");


fn main() {
    init_tools();
    gtk4::init().expect("failed to init gtk");

    let resource_data = glib::Bytes::from_static(RESOURCES_BYTES);
    let res = gio::Resource::from_data(&resource_data).unwrap();
    gio::resources_register(&res);

    let app = Application::new(Some("mystic.draw"), ApplicationFlags::FLAGS_NONE);
    app.set_resource_base_path(Some("/com/github/mkrueger/MysticDraw/"));
    app.connect_startup(|_| {
        adw::init();
    });
    app.connect_activate(|app| {
        MainWindow::build_ui(app);
    });
    app.run();
}

/*
    Tool ideas:

    <click> - click to set caret/normal typing.

    <paint> - paint with specific char/color

    <select> - Select Rectangle/Elipse/Triangle

    <brush> - brush <shape> with gradient,
        <solid brush>
        <color brush>
    <erase> - erase <shape> with gradient

    <draw shape>
        <line>
        <rectangle>
        <elipse>
        <triangle>

    <fill>
        <char>
        <color fg/bg>

    <font mode> - type with thedraw font
        <select font>
        <edit font>
        <select outline mode>
*/

