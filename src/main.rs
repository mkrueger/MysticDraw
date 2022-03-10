use gtk4::{gio::{ApplicationFlags, self}};
use libadwaita as adw;
use directories::{ ProjectDirs};
use adw::{prelude::*};
use gtk4::{Application};
use model::{init_tools, Editor, TextAttribute, Tool, TOOLS};
use ui::MainWindow;

mod model;
pub mod ui;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../data/font.fnt");

pub struct Settings {
    font_path: Option<std::path::PathBuf>,
    tab_size: i32
}

pub struct Workspace {
    pub settings: Settings,
    selected_tool: usize,
    selected_attribute: TextAttribute,

    pub show_fg_color: bool,
    pub show_bg_color: bool,

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
    settings: Settings { tab_size: 8, font_path: None},
    selected_tool: 0,
    selected_attribute: TextAttribute::DEFAULT,
    show_fg_color: true,
    show_bg_color: true,
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

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map_or(false, |s| s.starts_with('.'))
}
fn main() {
    let walker = walkdir::WalkDir::new("/home/mkrueger/Dokumente/AnsiArt").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            continue;
        }
        let extension = path.extension();
        if extension.is_none() { continue; }
        let extension = extension.unwrap().to_str();
        if extension.is_none() { continue; }
        let extension = extension.unwrap().to_lowercase();

        if extension == "xb" {
            println!("{}", path.to_str().unwrap());
        }
    }

    if let Some(proj_dirs) = ProjectDirs::from("github.com", "mkrueger",  "Mystic Draw") {
        unsafe {
            WORKSPACE.settings.font_path = Some(proj_dirs.data_dir().to_path_buf().join("fonts"));
        }
    }

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

