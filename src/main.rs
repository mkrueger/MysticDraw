use std::fs;

use gtk4::{gio::{ApplicationFlags, self}};
use libadwaita as adw;
use directories::{ ProjectDirs};
use adw::{prelude::*};
use gtk4::{Application};
use model::{init_tools, Editor, TextAttribute, Tool, TOOLS, Size};
use ui::MainWindow;

mod model;
pub mod ui;

pub struct Settings {
    font_path: Option<std::path::PathBuf>,
    console_font_path: Option<std::path::PathBuf>,
    tab_size: i32
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Grid {
    Off,
    Raster4x2,
    Raster6x3,
    Raster8x4,
    Raster12x6,
    Raster16x8
}


#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Guide {
    Off,
    Guide80x25,
    Guide80x40,
    Guide80x50,
    Guide44x22
}

pub struct Workspace {
    pub settings: Settings,
    selected_tool: usize,
    selected_attribute: TextAttribute,

    pub show_fg_color: bool,
    pub show_bg_color: bool,

    pub grid: Grid,
    pub guide: Guide
}

impl Workspace {
    pub fn cur_tool(&self) -> std::boxed::Box<&&'static mut dyn Tool> {
        unsafe {
            let t = &TOOLS[self.selected_tool];
            std::boxed::Box::new(t)
        }
    }

    pub fn get_grid_size(&self) -> Option<Size<u8>> {
        match self.grid {
            Grid::Off => None,
            Grid::Raster4x2 => Some(Size::from(4, 2)),
            Grid::Raster6x3 => Some(Size::from(6, 3)),
            Grid::Raster8x4 => Some(Size::from(8, 4)),
            Grid::Raster12x6 => Some(Size::from(12, 6)),
            Grid::Raster16x8 => Some(Size::from(16, 8)),
        }
    }

    pub fn get_guide_size(&self) -> Option<Size<u8>> {
        match self.guide {
            Guide::Off => None,
            Guide::Guide80x25 => Some(Size::from(80, 25)),
            Guide::Guide80x40 => Some(Size::from(80, 40)),
            Guide::Guide80x50 => Some(Size::from(80, 50)),
            Guide::Guide44x22 => Some(Size::from(44, 22)),
        }
    }
}

pub static mut WORKSPACE: Workspace = Workspace {
    settings: Settings { tab_size: 8, font_path: None, console_font_path: None},
    selected_tool: 0,
    selected_attribute: TextAttribute::DEFAULT,
    show_fg_color: true,
    show_bg_color: true,
    grid: Grid::Off,
    guide: Guide::Off
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

    if let Some(proj_dirs) = ProjectDirs::from("github.com", "mkrueger",  "Mystic Draw") {
        unsafe {
            WORKSPACE.settings.font_path = Some(proj_dirs.data_dir().to_path_buf().join("fonts/tdf"));
            WORKSPACE.settings.console_font_path = Some(proj_dirs.data_dir().to_path_buf().join("fonts/console"));

            if let Some(p) = &WORKSPACE.settings.font_path {
                fs::create_dir_all(p).expect("can't create tdf font path");
            }
            if let Some(p) = &WORKSPACE.settings.console_font_path {
                fs::create_dir_all(p).expect("can't create console font path");
            }
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

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::model::{Buffer, self};

    fn is_hidden(entry: &walkdir::DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map_or(false, |s| s.starts_with('.'))
    }
    
    fn comp(buf1: &Buffer, buf2: &Buffer) {
        assert_eq!(buf1.width, buf2.width);
        //assert_eq!(buf1.height, buf2.height);
    
        assert_eq!(buf1.title, buf2.title);
        assert_eq!(buf1.group, buf2.group);
        assert_eq!(buf1.author, buf2.author);
        assert_eq!(buf1.comments, buf2.comments);
        
        assert_eq!(buf1.palette.colors[0..16], buf2.palette.colors[0..16]);
    
        for y in 0..buf1.height {
            for x in 0..buf1.width {
                let pos = model::Position::from(x as i32, y as i32);
                let ch1 = buf1.get_char(pos);
                let ch2 = buf2.get_char(pos);
                if ch1.is_none() && ch2.is_none() { continue; }
                let ch1 = ch1.unwrap_or_default();
                let ch2 = ch2.unwrap_or_default();
    
                if ch1.is_transparent() && ch2.is_transparent() { continue; }
                if (ch1.char_code == b' ' || ch1.char_code == 0) && (ch2.char_code== b' ' || ch2.char_code== 0) && ch2.attribute.get_background() == ch2.attribute.get_background() { continue; }
                if ch1 != ch2 { 
                    println!("mismatch at y {} x {}", y, x);
                }
                assert_eq!(ch1, ch2);
            }
        }
    }
    
    // #[test]
    fn test_clear() {
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

            if extension == "ice" {
                let z = model::Buffer::load_buffer(path);
                if let Err(m) = z { 
                    eprintln!("Error loading file: {}", m);
                    continue;
                }
                let buf = z.unwrap();

                let mdf_bytes = model::convert_to_mdf(&buf).unwrap();
                let mut mdf_buffer = model::Buffer::new();
                model::read_mdf(&mut mdf_buffer, &mdf_bytes).unwrap();
                comp(&buf, &mdf_buffer);

                let adf_bytes = mdf_buffer.to_bytes(extension.as_str()).unwrap();
                let buf2 = Buffer::from_bytes(&std::path::PathBuf::from(path), &adf_bytes).unwrap();
                comp(&buf, &buf2);
            }
        }
    }

}
