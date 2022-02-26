use std::cell::RefCell;
use std::rc::Rc;

use gtk4::gio::ApplicationFlags;
use libadwaita as adw;

use adw::{prelude::*, TabBar, TabView};
use adw::{ApplicationWindow, HeaderBar};
use gtk4::{Application, Box, FileChooserAction, FileFilter, Orientation, ResponseType};
use model::{init_tools, Buffer, Editor, TextAttribute, Tool};
use ui::CharEditorView;

mod model;
mod ui;

pub const DEFAULT_FONT: &[u8] = include_bytes!("../data/font.fnt");

pub struct Workspace {
    selected_tool: usize,
    selected_attribute: TextAttribute,
    tools: Vec<&'static dyn Tool>,
    editors: Vec<Rc<RefCell<Editor>>>,
    tab_view: Option<TabView>,
}

impl Workspace {
    pub fn cur_tool(&self) -> std::boxed::Box<&'static dyn Tool> {
        let t = self.tools[self.selected_tool];
        std::boxed::Box::new(t)
    }

    pub fn get_tab_view(&self) -> &TabView {
        if let Some(view) = &self.tab_view {
            return view;
        }
        panic!("workspace not initialized!");
    }
}

pub static mut WORKSPACE: Workspace = Workspace {
    selected_tool: 0,
    selected_attribute: TextAttribute::DEFAULT,
    tools: Vec::new(),
    editors: Vec::new(),
    tab_view: None,
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

    // Create a new application
    let app = Application::new(Some("mystic.draw"), ApplicationFlags::FLAGS_NONE);
    app.connect_startup(|_| {
        adw::init();
    });

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

/*
fn read_a_file(file: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file)?;

    let mut result = Vec::new();
    file.read_to_end(&mut result)?;

    Ok(result)
}*/

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

fn add_tool(flow_box: &gtk4::FlowBox, nb: &gtk4::Notebook, tool: &dyn Tool) -> gtk4::ToggleButton {
    let button = gtk4::ToggleButton::builder()
        .icon_name(tool.get_icon_name())
        .build();
    flow_box.insert(&button, -1);
    let mut page_content = Box::new(Orientation::Vertical, 0);
    tool.add_tool_page(&mut page_content);

    let page_num = nb.append_page(&page_content, Option::<&gtk4::Widget>::None);

    button.connect_toggled(glib::clone!(@weak nb => move |_| {
        unsafe {
            WORKSPACE.selected_tool = page_num as usize;
        }
        nb.set_page(page_num as i32);
    }));
    button
}

fn construct_left_toolbar() -> Box {
    let result = Box::new(Orientation::Vertical, 0);

    result.append(&ui::ColorPicker::new());

    let flow_box = gtk4::FlowBox::builder()
        .valign(gtk4::Align::Start)
        .selection_mode(gtk4::SelectionMode::None)
        .build();

    let nb = gtk4::Notebook::builder().show_tabs(false).build();

    unsafe {
        let first = add_tool(&flow_box, &nb, WORKSPACE.tools[0]);
        for t in 1..WORKSPACE.tools.len() {
            add_tool(&flow_box, &nb, WORKSPACE.tools[t]).set_group(Some(&first));
        }
    }

    nb.set_page(0);
    result.append(&flow_box);
    result.append(&nb);
    result
}

fn construct_channels() -> Box {
    let result = Box::new(Orientation::Vertical, 0);
    let fg_button = gtk4::CheckButton::builder().label("Foreground").build();
    result.append(&fg_button);

    let bg_button = gtk4::CheckButton::builder().label("Foreground").build();

    result.append(&bg_button);

    result
}

fn construct_right_toolbar() -> Box {
    let result = Box::new(Orientation::Vertical, 0);

    let stack = gtk4::Stack::new();
    /*
    let page = stack.add_child(&construct_layer_view());
    page.set_name("page1");
    page.set_title("Layer");*/

    let page = stack.add_child(&construct_channels());
    page.set_name("page2");
    page.set_title("Channels");

    result.append(&stack);

    result
}

fn build_ui(app: &Application) {
    unsafe {
        let content = Box::new(Orientation::Vertical, 0);
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .content(&content)
            .build();

        let tab_view = TabView::builder().vexpand(true).build();
        WORKSPACE.tab_view = Some(tab_view);

        let title = adw::WindowTitle::builder().title("Mystic Draw").build();

        let hb = HeaderBar::builder()
            .title_widget(&title)
            .show_end_title_buttons(true)
            .build();
        let open_button = gtk4::Button::builder().label("Open").build();
        hb.pack_start(&open_button);

        let new_window_button = gtk4::Button::builder()
            .icon_name("tab-new-symbolic")
            .build();
        hb.pack_start(&new_window_button);

        let save_button = gtk4::Button::builder().label("Save").build();
        hb.pack_end(&save_button);
        hb.pack_end(
            &gtk4::MenuButton::builder()
                .icon_name("open-menu-symbolic")
                .build(),
        );

        content.append(&hb);
        let tab_box = Box::new(Orientation::Vertical, 0);
        let tab_bar = TabBar::builder().view(WORKSPACE.get_tab_view()).build();
        tab_box.append(&tab_bar);
        tab_box.append(WORKSPACE.get_tab_view());

        let right_pane = gtk4::Paned::builder()
            .orientation(Orientation::Horizontal)
            .start_child(&tab_box)
            .end_child(&construct_right_toolbar())
            .build();
        right_pane.set_position(200);

        let left_pane = Box::new(Orientation::Horizontal, 0);
        left_pane.append(&construct_left_toolbar());
        left_pane.append(&right_pane);
        content.append(&left_pane);

        open_button.connect_clicked(glib::clone!(@weak window => move |_| {

        let filter = FileFilter::new();
        filter.add_pattern("*.ans");
        filter.add_pattern("*.avt");
        filter.add_pattern("*.bin");
        filter.add_pattern("*.xb");
        filter.add_pattern("*.pcb");
        filter.add_pattern("*.asc");
        filter.add_pattern("*.nfo");
        filter.add_pattern("*.diz");

        let file_chooser = gtk4::FileChooserDialog::builder()
            .title("Open file")
            .action(FileChooserAction::Open)
            .filter(&filter)
            .transient_for(&window)
            .width_request(640)
            .height_request(480)
            .build();

        file_chooser.add_button("Open", ResponseType::Ok);
        file_chooser.add_button("Cancel", ResponseType::Cancel);

        file_chooser.connect_response(move |d: &gtk4::FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let filename = file.path().expect("Couldn't get file path");
                let buffer = Buffer::load_buffer(&filename.as_path().to_path_buf());
                if let Ok(buf) = buffer {
                    load_page(WORKSPACE.get_tab_view(), buf);
                    std::env::set_current_dir(filename.parent().unwrap()).expect("can't set current path.");
                }
            }
            d.close();
        });
        file_chooser.show();
    }));

        save_button.connect_clicked(glib::clone!(@weak window  => move |_| {
            save_as(&window, WORKSPACE.get_tab_view());
        }));

        new_window_button.connect_clicked(glib::clone!(@weak window => move |_| {
            let mut buffer = Buffer::new();
            buffer.width  = 80;
            buffer.height = 25;
            load_page(WORKSPACE.get_tab_view(), buffer);
        }));

        window.present();
    }
}

fn save_as(window: &ApplicationWindow, tab_view: &'static TabView) {
    let filter = FileFilter::new();
    filter.add_pattern("*.ans");
    filter.add_pattern("*.avt");
    filter.add_pattern("*.bin");
    filter.add_pattern("*.xb");
    filter.add_pattern("*.pcb");
    filter.add_pattern("*.asc");
    filter.add_pattern("*.nfo");
    filter.add_pattern("*.diz");

    let file_chooser = gtk4::FileChooserDialog::builder()
        .title("Save file")
        .action(FileChooserAction::Open)
        .filter(&filter)
        .transient_for(window)
        .width_request(640)
        .height_request(480)
        .build();

    file_chooser.add_button("Save", ResponseType::Ok);
    file_chooser.add_button("Cancel", ResponseType::Cancel);

    // let path = std::env::current_dir().unwrap();
    file_chooser.connect_response(move |d: &gtk4::FileChooserDialog, response: ResponseType| {
        if response == ResponseType::Ok {
            if let Some(page) = tab_view.selected_page() {
                unsafe {
                    let file = d.file().expect("Couldn't get file");
                    let filename = file.path().expect("Couldn't get file path");

                    let id = page.data::<usize>("editor_id");
                    let id = *id.unwrap().as_ptr();
                    for e in &WORKSPACE.editors {
                        if e.borrow().id == id {
                            e.borrow().save_content(&filename);
                        }
                    }
                }
            }
        }
        d.close();
    });
    file_chooser.show();
}

static mut EDITOR_ID: usize = 0;

fn load_page(tab_view: &TabView, buf: Buffer) {
    let child2 = CharEditorView::new();
    let scroller = gtk4::ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .child(&child2)
        .build();
    let page = tab_view.add_page(&scroller, None);
    let file_name = buf.file_name.clone();
    unsafe {
        let editor = Editor::new(EDITOR_ID, buf);
        page.set_data("editor_id", EDITOR_ID);
        EDITOR_ID += 1;

        let handle = Rc::new(RefCell::new(editor));
        WORKSPACE.editors.push(handle.clone());
        child2.set_editor_handle(handle);
        if let Some(x) = file_name {
            let fin = x
                .as_path()
                .file_name()
                .ok_or_else(|| panic!("Can't convert file name"))
                .unwrap();
            page.set_title(fin.to_str().unwrap());
        }
        tab_view.set_selected_page(&page);
        child2.grab_focus();
    }
}
