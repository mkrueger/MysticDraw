use std::ptr;

use editor::Editor;
use libadwaita as adw;

use adw::{prelude::*, TabBar, TabView};
use adw::{ApplicationWindow, HeaderBar};
use gtk4::{Application, Box, FileChooserAction, Orientation, ResponseType};

use crate::model::Buffer;

mod editor;
mod gtk_view;
mod model;
mod sauce;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../data/font.fnt");

struct Workspace {
    open_editors: Vec<Editor>,
}

impl Workspace {
    pub fn get_editor(id: usize) -> &'static Editor {
        unsafe { &WORKSPACE.open_editors[id] }
    }
    pub fn open_editor(buf: Buffer) -> usize {
        unsafe {
            WORKSPACE.open_editors.push(Editor::new(buf));
            WORKSPACE.open_editors.len() - 1
        }
    }
}

static mut WORKSPACE: Workspace = Workspace {
    open_editors: Vec::new(),
};

fn main() {
    // Load GL pointers from epoxy (GL context management library used by GTK).
    {
        #[cfg(target_os = "macos")]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
        #[cfg(all(unix, not(target_os = "macos")))]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
        #[cfg(windows)]
        let library = libloading::os::windows::Library::open_already_loaded("epoxy-0.dll").unwrap();

        epoxy::load_with(|name| {
            unsafe { library.get::<_>(name.as_bytes()) }
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null())
        });
    }

    // Create a new application
    let app = Application::builder().application_id("mystic.draw").build();
    app.connect_startup(|_| {
        adw::init();
    });

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

static OPEN_BUTTONS: [(&str, ResponseType); 2] = [
    ("_Cancel", ResponseType::Cancel),
    ("_Open", ResponseType::Ok),
];

/*
fn read_a_file(file: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file)?;

    let mut result = Vec::new();
    file.read_to_end(&mut result)?;

    Ok(result)
}*/

fn build_ui(app: &Application) {
    let content = Box::new(Orientation::Vertical, 0);
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(350)
        .content(&content)
        .build();

    let tab_view = TabView::builder().vexpand(true).build();

    let title = adw::WindowTitle::builder().title("Mystic Draw").build();

    let hb = HeaderBar::builder()
        .title_widget(&title)
        .show_end_title_buttons(true)
        .build();
    let open_button = gtk4::Button::builder().label("Open").build();

    hb.pack_start(&open_button);
    hb.pack_start(
        &gtk4::Button::builder()
            .icon_name("tab-new-symbolic")
            .build(),
    );

    hb.pack_end(&gtk4::Button::builder().label("Save").build());
    hb.pack_end(
        &gtk4::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .build(),
    );

    content.append(&hb);

    let tab_bar = TabBar::builder().view(&tab_view).build();
    content.append(&tab_bar);
    content.append(&tab_view);

    window.present();

    open_button.connect_clicked(glib::clone!(@weak window, @weak tab_view => move |_| {
        let file_chooser = gtk4::FileChooserDialog::new(Some("Open ansi file"), Some(&window), FileChooserAction::Open, &OPEN_BUTTONS);

        file_chooser.connect_response(move |d: &gtk4::FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let filename = file.path().expect("Couldn't get file path");
                let buffer = Buffer::load_buffer(filename.as_path().to_path_buf());
                if let Ok(buf) = buffer {
                    load_page(&tab_view, buf);
                }
            }
            d.close();
        });
        file_chooser.show();
    }));
}

fn load_page(tab_view: &TabView, buf: Buffer) {
    let child2 = gtk_view::CharEditorView::new();
    let scroller = gtk4::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&child2)
        .build();

    let page = tab_view.add_page(&scroller, None);

    let fin = (*buf.file_name)
        .as_path()
        .file_name()
        .ok_or_else(|| panic!("Can't convert file name"))
        .unwrap();
    page.set_title(fin.to_str().unwrap());

    let id = Workspace::open_editor(buf);
    child2.set_buffer(id);

    tab_view.set_selected_page(&page);
}
