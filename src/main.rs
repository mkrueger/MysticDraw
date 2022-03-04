use gtk4::{gio::{ApplicationFlags}};
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
    selected_attribute: TextAttribute
}

impl Workspace {
    pub fn cur_tool(&self) -> std::boxed::Box<&&'static mut dyn Tool> {
        unsafe {
            let t = &TOOLS[self.selected_tool];
            std::boxed::Box::new(t)
        }
    }
}

pub static mut WORKSPACE: Workspace = Workspace {
    selected_tool: 0,
    selected_attribute: TextAttribute::DEFAULT
};

pub fn sync_workbench_state(editor: &mut Editor) {
    // quite lame but unfortunately I don't see a sane way to really work
    // with the same state accross everything I'm not able to get any mutable data strucutures out of Gtk
    // and working with weird RefCell/Cell/Rc makes things worse than doing a manualy sync.
    unsafe {
        editor.cursor.attr = WORKSPACE.selected_attribute;
    }
}

fn main() {
    init_tools();
    let app = Application::new(Some("mystic.draw"), ApplicationFlags::FLAGS_NONE);
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

