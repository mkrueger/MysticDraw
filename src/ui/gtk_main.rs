use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use glib::{clone};
use gtk4::gio::SimpleAction;
use libadwaita as adw;

use adw::{prelude::*, TabBar, TabPage, TabView};
use adw::{ApplicationWindow, HeaderBar};
use gtk4::{Application, Box, FileChooserAction, Orientation, ResponseType, MessageType, ButtonsType, DialogFlags};

use crate::WORKSPACE;
use crate::model::{Buffer, DosChar, Editor, Position, TextAttribute, Tool, TOOLS, Layer};

use super::{AnsiView, ColorPicker, layer_view};

pub struct MainWindow {
    pub window: ApplicationWindow,
    tab_view: TabView,
    color_picker: ColorPicker,
    tab_to_view: RefCell<HashMap<Rc<TabPage>, Rc<AnsiView>>>,
    title: adw::WindowTitle,

    layer_listbox_model: layer_view::Model,
    layer_listbox: gtk4::ListBox,
    tool_container_box: gtk4::FlowBox,
    tool_notebook: gtk4::Notebook,
    fg_button: gtk4::CheckButton,
    bg_button: gtk4::CheckButton,

    mini_map: AnsiView
}

#[derive(Clone, Debug)]
pub struct ClipboardLayer {
    pub layer: crate::model::Layer,
    pub size: crate::model::Size
}

impl MainWindow {
    pub fn build_ui(app: &Application) {
        let content = Box::new(Orientation::Vertical, 0);
        let (title, header_bar) = MainWindow::construct_titlebar();

        let main_window = Rc::new(MainWindow {
            window: ApplicationWindow::builder()
                .application(app)
                .default_width(1024)
                .default_height(568)
                .content(&content)
                .build(),
            tab_view: TabView::builder().vexpand(true).build(),
            color_picker: ColorPicker::new(),
            tab_to_view: Default::default(),
            title,
            layer_listbox_model: layer_view::Model::new(),
            layer_listbox: gtk4::ListBox::new(),
            tool_container_box: gtk4::FlowBox::builder()
                .valign(gtk4::Align::Start)
                .selection_mode(gtk4::SelectionMode::None)
                .build(),
            tool_notebook: gtk4::Notebook::builder()
                .show_tabs(false)
                .vexpand(true)
                .build(),
            fg_button: gtk4::CheckButton::builder()
                .label("Foreground")
                .active(true)
                .build(),
            bg_button: gtk4::CheckButton::builder()
                .label("Background")
                .active(true)
                .build(),
            mini_map: AnsiView::new()
        });

        content.append(&header_bar);
        let tab_box = Box::new(Orientation::Vertical, 0);
        let tab_bar = TabBar::builder().view(&main_window.tab_view).build();
        tab_box.append(&tab_bar);
        tab_box.append(&main_window.tab_view);

        let right_pane = gtk4::Paned::builder()
            .orientation(Orientation::Horizontal)
            .start_child(&tab_box)
            .resize_end_child(false)
            .end_child(&main_window.construct_right_toolbar())
            .build();
        right_pane.set_position(1024 - 380);

        let left_pane = Box::new(Orientation::Horizontal, 0);
        left_pane.append(&main_window.construct_left_toolbar());
        left_pane.append(&right_pane);
        content.append(&left_pane);
        main_window.window.present();

        main_window.tab_view.connect_selected_page_notify(
            clone!(@strong main_window => move |_| {
                main_window.page_swap();
            }),
        );

        if let Some(e) = main_window.get_current_editor() {
            e.borrow_mut().cur_layer = 1;
        }
        main_window.tool_notebook.connect_switch_page(clone!(@strong main_window => move |_, _, _| {
            main_window.update_editor();
        }));

        main_window.layer_listbox.connect_row_selected(clone!(@strong main_window => move |_, row| {
            if let Some(row) = row {
                let idx = row.index();
                if let Some(e) = main_window.get_current_editor() {
                    e.borrow_mut().cur_layer = idx;
                }
            }   
        }));
        main_window.fg_button.connect_toggled(clone!(@weak main_window => move |b| {
            unsafe {
                WORKSPACE.show_fg_color = b.is_active();
                main_window.update_editor();
            }
        }));

        main_window.bg_button.connect_toggled(clone!(@weak main_window => move |b| {
            unsafe {
                WORKSPACE.show_bg_color = b.is_active();
                main_window.update_editor();
            }
        }));

        main_window.layer_listbox.set_activate_on_single_click(false);
        main_window.layer_listbox.connect_row_activated(clone!(@strong main_window => move |_, row| {
            let idx = row.index();
            if let Some(e) = main_window.get_current_editor() {
                let res = Rc::new(layer_view::display_edit_layer_dialog(&main_window.window, &e.borrow_mut().buf.layers[idx as usize]));
                let rd = &res.clone().open_button;
                rd.connect_clicked(clone!(@strong main_window => move |_| {
                    res.set_layer_values(&mut e.borrow_mut().buf.layers[idx as usize]);
                    res.dialog.close();
                    main_window.update_layer_view();
                    main_window.update_editor();
                }));
            }
        }));

        {
            //  let rc = rc.clone();
            let open_action = SimpleAction::new("new", None);
            open_action.connect_activate(clone!(@strong main_window => move |_,_| {

                let nfd = crate::ui::new_file_dialog::display_newfile_dialog(&main_window);
                let ws = Rc::new(nfd.width_spin_button);
                let hs = Rc::new(nfd.height_spin_button);
                
                nfd.open_button.connect_clicked(clone!(@strong main_window => move |_| {
                    let mut buffer = Buffer::create(ws.value() as u16, hs.value() as u16);
                    buffer.file_name = None;
                    let editor = main_window.load_page(buffer);
                    editor.borrow_mut().request_refresh = std::boxed::Box::new(clone!(@strong main_window => move || {
                        main_window.update_editor();
                    }));
                    main_window.update_layer_view();
                    nfd.dialog.close();
                    main_window.update_layer_view();
                    main_window.update_editor();

                }));
            }));
            app.add_action(&open_action);
        }

        {
            let open_action = SimpleAction::new("open", None);
            open_action.connect_activate(clone!(@strong main_window => move |_,_| {

                let file_chooser = gtk4::FileChooserDialog::builder()
                    .title("Open file")
                    .action(FileChooserAction::Open)
                    .transient_for(&main_window.window)
                    .modal(true)
                    .width_request(640)
                    .height_request(480)
                    .vexpand(false)
                    .hexpand(false)
                    .build();

                file_chooser.add_button("Open", ResponseType::Ok);
                file_chooser.add_button("Cancel", ResponseType::Cancel);

                file_chooser.connect_response(clone!(@strong main_window => move |d, response| {
                    if response == ResponseType::Ok {
                        let file = d.file().expect("Couldn't get file");
                        let file_name = file.path().expect("Couldn't get file path");
                        d.close();

                        let res  = Buffer::load_buffer(file_name.as_path());
                        if let Err(err) = res  {
                            main_window.show_error(format!("Error opening file '{}'", file_name.as_os_str().to_string_lossy()), Some(err.to_string().as_str()));
                            return;
                        }

                        let editor = main_window.load_page(res.unwrap());
                        editor.borrow_mut().request_refresh = std::boxed::Box::new(clone!(@strong main_window => move || {
                            main_window.update_editor();
                        }));
                        std::env::set_current_dir(file_name.parent().unwrap()).expect("can't set current path.");
                    }
                    d.close();
                }));
                file_chooser.present();

            }));
            app.add_action(&open_action);
        }

        {
            let open_action = SimpleAction::new("save", None);
            open_action.connect_activate(clone!(@weak main_window => move |_,_| {
                if let Some(editor) = main_window.get_current_editor() {
                    if let Some(file_name) = &editor.borrow().buf.file_name {
                        main_window.handle_error(editor.borrow().save_content(file_name), move || format!("Error saving {}", file_name.as_os_str().to_string_lossy()));
                        return;
                    }
                } else {
                    return;
                }

                let file_chooser = gtk4::FileChooserDialog::builder()
                    .title("Save file")
                    .action(FileChooserAction::Save)
                    .transient_for(&main_window.window)
                    .modal(true)
                    .width_request(640)
                    .height_request(480)
                    .build();

                file_chooser.add_button("Save", ResponseType::Ok);
                file_chooser.add_button("Cancel", ResponseType::Cancel);
                file_chooser.connect_response(clone!(@weak main_window => move |d, response| {
                    if response == ResponseType::Ok {
                        if let Some(page) = main_window.get_current_ansi_view() {
                            let file = d.file().expect("Couldn't get file");
                            let filename = file.path().expect("Couldn't get file path");
                            d.close();
                            page.get_editor().borrow_mut().buf.file_name = Some(filename.clone());
                            main_window.handle_error(page.get_editor().borrow().save_content(&filename), move || format!("Error saving {}", filename.as_os_str().to_string_lossy()));
                            (page.get_editor().borrow().buf.file_name_changed)()
                        } else {
                            eprintln!("can't find ansi view to save.");
                        }
                    }
                    main_window.page_swap();
                    d.close();
                }));
                file_chooser.show();
            }));
            app.add_action(&open_action);
        }

        {
            let open_action = SimpleAction::new("saveas", None);
            open_action.connect_activate(clone!(@strong main_window => move |_,_| {
                let file_chooser = gtk4::FileChooserDialog::builder()
                    .title("Save file")
                    .action(FileChooserAction::Save)
                    .transient_for(&main_window.window)
                    .modal(true)
                    .width_request(640)
                    .height_request(480)
                    .build();

                file_chooser.add_button("Save", ResponseType::Ok);
                file_chooser.add_button("Cancel", ResponseType::Cancel);
                file_chooser.connect_response(clone!(@weak main_window => move |d, response| {
                    if response == ResponseType::Ok {
                        if let Some(page) = main_window.get_current_ansi_view() {
                            let file = d.file().expect("Couldn't get file");
                            let filename = file.path().expect("Couldn't get file path");
                            page.get_editor().borrow_mut().buf.file_name = Some(filename.clone());
                            main_window.handle_error(page.get_editor().borrow().save_content(&filename), move || format!("Error saving {}", filename.as_os_str().to_string_lossy()));
                            (page.get_editor().borrow().buf.file_name_changed)()
                        } else {
                            eprintln!("can't find ansi view to save.");
                        }
                    }
                    main_window.page_swap();
                    d.close();
                }));
                file_chooser.show();
            }));
            app.add_action(&open_action);
        }

        { // layer actions
            let action = SimpleAction::new("layer-new", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                if let Some(editor) = main_window.get_current_editor() {
                    let mut new_layer = crate::model::Layer::new();
                    new_layer.title = "New layer".to_string();
                    editor.borrow_mut().buf.layers.insert(0, new_layer);
                    main_window.update_layer_view();

                    let row = main_window.layer_listbox.row_at_index(0);
                    if let Some(row)= row {
                        main_window.layer_listbox.select_row(Some(&row));
                    }
                }
                main_window.update_editor();
            }));
            app.add_action(&action);
            let action = SimpleAction::new("layer-up", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                let cur = main_window.get_current_ansi_view();
                if let Some(editor) = cur.map(|view| view.get_editor()) {
                    if let Some(row) = main_window.layer_listbox.selected_row() {
                        let idx = row.index() as usize;
                        if idx > 0 {
                            editor.borrow_mut().buf.layers.swap(idx, idx - 1);
                            main_window.update_layer_view();
                            let row = main_window.layer_listbox.row_at_index(idx as i32 - 1);
                            if let Some(row)= row {
                                main_window.layer_listbox.select_row(Some(&row));
                            }
                        }
                    }
                }
                main_window.update_editor();
            }));
            app.add_action(&action);
            let action = SimpleAction::new("layer-down", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                let cur = main_window.get_current_ansi_view();
                if let Some(editor) = cur.map(|view| view.get_editor()) {
                    if let Some(row) = main_window.layer_listbox.selected_row() {
                        let idx = row.index() as usize;
                        let len = editor.borrow().buf.layers.len();
                        if idx + 1 < len {
                            editor.borrow_mut().buf.layers.swap(idx, idx + 1);
                            main_window.update_layer_view();
                            let row = main_window.layer_listbox.row_at_index(idx as i32 + 1);
                            if let Some(row)= row {
                                main_window.layer_listbox.select_row(Some(&row));
                            }
                        }
                    }
                }
                main_window.update_editor();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("layer-copy", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                if let Some(editor) = main_window.get_current_editor() {
                    let row = main_window.layer_listbox.row_at_index(0);
                    if let Some(row)= row {
                        let idx = row.index();
                        let mut new_layer= editor.borrow_mut().buf.layers[idx as usize].clone();
                        new_layer.title = format!("{} copy", new_layer.title);
                        editor.borrow_mut().buf.layers.insert(0, new_layer);
                        main_window.update_layer_view();

                        main_window.layer_listbox.select_row(Some(&row));
                    }
                }
                main_window.update_editor();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("layer-delete", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                let cur = main_window.get_current_ansi_view();
                if let Some(editor) = cur.map(|view| view.get_editor()) {
                    if let Some(row) = main_window.layer_listbox.selected_row() {
                        let idx = row.index();
                        editor.borrow_mut().buf.layers.remove(idx as usize);
                        main_window.update_layer_view();
                    }
                }
                main_window.update_editor();
            }));
            app.add_action(&action);
            
            let action = SimpleAction::new("cut", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                main_window.cut_to_clipboard();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("copy", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                main_window.copy_to_clipboard();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("paste", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                main_window.paste_from_clipboard();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("preferences", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                let dialog = super::display_settings_dialog(&main_window);
                dialog.open_button.connect_clicked(clone!(@strong main_window => move |_| {
                   
                    dialog.dialog.close();
                    main_window.update_layer_view();
                    main_window.update_editor();
                }));

            }));
            app.add_action(&action);

            let action = SimpleAction::new("bugreport", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                let url = "https://github.com/mkrueger/MysticDraw/issues";
                if let Err(err) = open::that(url) {
                    main_window.show_error(format!("Error opening url '{}'", url), Some(err.to_string().as_str()));
                }
            }));
            app.add_action(&action);
      
            let action = SimpleAction::new("undo", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                if let Some(editor) = main_window.get_current_editor() {
                    editor.borrow_mut().undo();
                 }
                 main_window.update_editor();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("redo", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                if let Some(editor) = main_window.get_current_editor() {
                    editor.borrow_mut().redo();
                 }
                 main_window.update_editor();
            }));
            app.add_action(&action);

            let action = SimpleAction::new("erase", None);
            action.connect_activate(clone!(@strong main_window => move |_,_| {
                if let Some(editor) = main_window.get_current_editor() {
                    if editor.borrow().cur_selection.is_some() {
                        editor.borrow_mut().delete_selection();
                    } else {
                        editor.borrow_mut().clear_cur_layer();
                    }
                }
                 main_window.update_editor();
            }));
            app.add_action(&action);

            app.set_accels_for_action("app.open", &["<primary>o"]);
            app.set_accels_for_action("app.preferences", &["<primary>comma"]);
            app.set_accels_for_action("app.cut", &["<primary>x"]);
            app.set_accels_for_action("app.copy", &["<primary>c"]);
            app.set_accels_for_action("app.paste", &["<primary>v"]);
            app.set_accels_for_action("app.undo", &["<primary>z"]);
            app.set_accels_for_action("app.redo", &["<Primary><Shift>z"]);
        }
    }

    fn show_error(&self, error: String, secondary_text: Option<&str>)
    {
        let dialog = gtk4::MessageDialog::new(Some(&self.window), DialogFlags::DESTROY_WITH_PARENT, MessageType::Error, ButtonsType::Close, error.as_str());
        dialog.add_button("Send bug report", ResponseType::Help);

        dialog.set_secondary_text(secondary_text);
        dialog.show();
        dialog.connect_response(move |d, r| {
            d.destroy();
            if r == ResponseType::Help {
                let url = "https://github.com/mkrueger/MysticDraw/issues";
                if open::that(url).is_err() {
                    eprintln!("You're kidding me");
                }
            }
        });
    }
    fn handle_error<T, E: std::fmt::Display, F>(&self, error: std::result::Result<T, E>, get_title: F) 
        where F: Fn()->String {
        if let Err(e) = error {
            self.show_error(get_title(), Some(e.to_string().as_str()));
        }

    }
    fn paste_from_clipboard(&self) -> bool {
        unsafe {
            if !crate::WORKSPACE.cur_tool().use_selection() || !crate::WORKSPACE.cur_tool().use_caret()  { return false; }
        }
        let cur = self.get_current_ansi_view();
        if let Some(editor) = cur.map(|view| view.get_editor()) {

            let display = gtk4::gdk::Display::default().unwrap();
            let clipboard = display.clipboard();
            unsafe {
                if let Some(data) = clipboard.data::<ClipboardLayer>("MysticDraw.Layer") {
                    let layer = data.as_ref();
                    let mut opos = Position::new();
                    let mut pos = editor.borrow_mut().get_cursor_position();
                    let x1 = pos.x;
                    editor.borrow_mut().begin_atomic_undo();
                    for _ in 0..layer.size.height {
                        for _ in 0..layer.size.width {
                            editor.borrow_mut().set_char(pos, layer.layer.get_char(opos));
                            pos.x += 1;
                            opos.x += 1;
                        }
                        pos.y += 1;
                        opos.y += 1;
                        opos.x = 0;
                        pos.x = x1;
                    }
                    editor.borrow_mut().end_atomic_undo();
                    self.update_editor();
                    return true;
                }
            }
        }
        false
    }

    fn cut_to_clipboard(&self) {
        if !self.copy_to_clipboard() { return; }
        let cur = self.get_current_ansi_view();

        if let Some(editor) = cur.map(|view| view.get_editor()) {
            let pos = editor.borrow().cur_selection.as_ref().unwrap().rectangle.start;
            editor.borrow_mut().begin_atomic_undo();
            if editor.borrow().cur_selection.is_some() {
                editor.borrow_mut().delete_selection();
            }
            editor.borrow_mut().set_cursor_position(pos);
            editor.borrow_mut().end_atomic_undo();
        } 
    }

    fn copy_to_clipboard(&self) -> bool {
        unsafe {
            if !crate::WORKSPACE.cur_tool().use_selection() || !crate::WORKSPACE.cur_tool().use_caret()  { return false; }
        }
        let cur = self.get_current_ansi_view();
        if let Some(editor) = cur.map(|view| view.get_editor()) {
            if let Some(selection) = &editor.borrow().cur_selection {
                let mut pos = selection.rectangle.start;
                let mut opos = Position::new();

                let mut copy_layer = ClipboardLayer { layer: Layer::new(), size: selection.rectangle.size };

                for _ in 0..copy_layer.size.height {
                    for _ in 0..copy_layer.size.width {
                        copy_layer.layer.set_char(opos, editor.borrow().get_char(pos));
                        pos.x += 1;
                        opos.x += 1;
                    }
                    pos.y += 1;
                    opos.y += 1;
                    opos.x = 0;
                    pos.x = selection.rectangle.start.x;
                }

                let display = gtk4::gdk::Display::default().unwrap();
                let clipboard = display.clipboard();
                unsafe {
                    clipboard.set_data("MysticDraw.Layer", copy_layer);
                }
                self.get_current_ansi_view().unwrap().queue_draw();
                self.mini_map.queue_draw();
                return true;
            }
        }
        false
   }

    fn page_swap(&self) {
        let cur = self.get_current_ansi_view();
        self.layer_listbox_model.clear();

        if cur.is_none() {
            self.title.set_title("Mystic Draw");
            self.title.set_subtitle("");
            return;
        }
        if let Some(view) = cur {
            let editor = view.get_editor();
            self.color_picker.set_editor(&editor);
            let fn_opt = &(editor.borrow().buf.file_name);
            if fn_opt.is_none() {
                self.title.set_title("Untitled");
                self.title.set_subtitle("");
            } else if let Some(name) = fn_opt {
                let file = name.file_name().unwrap().to_str().unwrap();
                self.title.set_title(file);

                let path = name.parent().unwrap().to_str().unwrap();
                self.title.set_subtitle(path);
            }
            self.mini_map.set_editor_handle(editor.clone());
            self.mini_map.queue_draw();
        }
        self.update_layer_view();
    }

    pub fn update_layer_view(&self)
    {
        self.layer_listbox_model.clear();
        if let Some(editor) = self.get_current_editor() {
            for b in &editor.borrow().buf.layers {
                self.layer_listbox_model.append(&layer_view::RowData::new(&b.title, b.is_visible));
            }
            for i in 0..editor.borrow().buf.layers.len() {
                let row = self.layer_listbox.row_at_index(i as i32);
                if let Some(row)= row {
                    row.connect_local("isvisiblechanged", false,clone!(@strong editor => @default-return None, move |args| {
                        let row = args.get(0).unwrap().get::<gtk4::ListBoxRow>().unwrap();
                        let is_visible = args.get(1).unwrap().get::<bool>().unwrap();
                        if let Some(layer) = editor.borrow_mut().buf.layers.get_mut(row.index() as usize) {
                            layer.is_visible = is_visible;
                        }
                        (editor.borrow().request_refresh)();
                        None
                    }));
                }
            }
            let len = editor.borrow().buf.layers.len();
            if len > 0 {
                let row = self.layer_listbox.row_at_index(0);
                if let Some(row)= row {
                    self.layer_listbox.select_row(Some(&row));
                }
            }
        }
        self.tool_notebook.queue_draw();
    }

    pub fn update_editor(&self)
    {
        if let Some(view) = &self.get_current_ansi_view() {
            view.queue_draw();
            self.mini_map.queue_draw();
        }
    }

    pub fn get_current_editor(&self) -> Option<Rc<RefCell<Editor>>> {
        let cur = self.get_current_ansi_view();
        cur.map(|view| view.get_editor())
    }

    pub fn get_current_ansi_view(&self) -> Option<Rc<AnsiView>> {
        if let Some(page) = self.tab_view.selected_page() {
            if let Some(w) = self.tab_to_view.borrow().get(&page) {
                return Some(w.clone());
            }
        }
        None
    }

    fn construct_titlebar() -> (adw::WindowTitle, HeaderBar) {
        let title = adw::WindowTitle::builder().title("Mystic Draw").build();
        let hb = HeaderBar::builder()
            .title_widget(&title)
            .show_end_title_buttons(true)
            .build();
        let open_button = gtk4::Button::builder()
            .label("Open")
            .action_name("app.open")
            .build();
        hb.pack_start(&open_button);

        let new_window_button = gtk4::Button::builder()
        .icon_name("tab-new-symbolic")
        .action_name("app.new")
        .build();
        hb.pack_start(&new_window_button);

        let menu = gtk4::gio::Menu::new();
        menu.append(Some("Save as…"), Some("app.saveas"));
        menu.append(Some("Preferences"), Some("app.preferences"));
        menu.append(Some("Keyboard Map"), Some("app.keymap"));
        menu.append(Some("Send bug report"), Some("app.bugreport"));
        menu.append(Some("About"), Some("app.about"));
        hb.pack_end(
            &gtk4::MenuButton::builder()
                .icon_name("open-menu-symbolic")
                .menu_model(&menu)
                .build(),
        );
        let save_button = gtk4::Button::builder()
            .label("Save")
            .action_name("app.save")
            .build();
        hb.pack_end(&save_button);

        (title, hb)
    }

    fn construct_left_toolbar(&self) -> Box {
        let result = Box::new(Orientation::Vertical, 0);
        result.set_hexpand(false);
        result.set_width_request(200);
        result.append(&self.color_picker);
        unsafe {
            let first = self.add_tool(TOOLS[0]);
            for t in TOOLS.iter().skip(1) {
                self.add_tool(*t).set_group(Some(&first));
            }
            first.set_active(true);
        }
        self.tool_notebook.set_page(0);
        result.append(&self.tool_container_box);
        result.append(&self.tool_notebook);
        result
    }

    fn construct_right_toolbar(&self) -> gtk4::Box {

        self.mini_map.set_mimap_mode(true);
        let scroller = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .min_content_width(10)
            .min_content_height(10)
            .child(&self.mini_map)
            .build();

        let switcher = gtk4::StackSwitcher::new();
        let stack = gtk4::Stack::new();
        
        let page = stack.add_child(&self.construct_layer_view());
        page.set_name("page1");
        page.set_title("Layer");

        let page = stack.add_child(&self.construct_channels());
        page.set_name("page2");
        page.set_title("Channels");
        
        switcher.set_stack(Some(&stack));
        let layer_box = Box::new(Orientation::Vertical, 0);
        layer_box.append(&switcher);
        layer_box.append(&stack);

        let right_pane = gtk4::Paned::builder()
            .orientation(Orientation::Vertical)
            .start_child(&scroller)
            .resize_end_child(false)
            .end_child(&layer_box)
            .build();
            right_pane.set_position(1024 - 380);

        let layer_box = Box::new(Orientation::Vertical, 0);
        layer_box.set_vexpand(false);

        layer_box.append(&right_pane);

        layer_box
    }

    fn construct_layer_view(&self) -> gtk4::Box {
        self.layer_listbox.bind_model(
            Some(&self.layer_listbox_model), // 
            clone!(@strong self as window => @default-panic, move |item| {
                layer_view::ListBoxRow::new(
                    item.downcast_ref::<layer_view::RowData>()
                        .expect("RowData is of wrong type"),
                )
                .upcast::<gtk4::Widget>()
            }),
        );

        let scrolled_window = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
            .vexpand(true)
            .child(&self.layer_listbox)
            .build();
        
        let toolbar = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(5)
            .vexpand(false)
            .build();
        toolbar.style_context().add_class("toolbar");
        let new_layer_button = gtk4::Button::builder()
            .icon_name("md-layer-add")
            .action_name("app.layer-new")
            .build();
        toolbar.append(&new_layer_button);

        let layer_up_button = gtk4::Button::builder()
            .icon_name("md-layer-up")
            .action_name("app.layer-up")
            .build();
        toolbar.append(&layer_up_button);

        let layer_down_button = gtk4::Button::builder()
            .icon_name("md-layer-down")
            .action_name("app.layer-down")
            .build();
        toolbar.append(&layer_down_button);

        let layer_copy_button = gtk4::Button::builder()
            .icon_name("md-layer-copy")
            .action_name("app.layer-copy")
            .build();
        
        toolbar.append(&layer_copy_button);

        let layer_delete_button = gtk4::Button::builder()
            .icon_name("md-layer-delete")
            .action_name("app.layer-delete")
            .build();
        toolbar.append(&layer_delete_button);

        let result = Box::new(Orientation::Vertical, 0);
        result.append(&scrolled_window);
        result.append(&toolbar);
        result
    }

    fn construct_channels(&self) -> Box {
        let result = Box::new(Orientation::Vertical, 0);
        result.append(&self.fg_button);
        result.append(&self.bg_button);
        
        result
    }

    fn add_tool(
        &self,
        tool: &dyn Tool,
    ) -> gtk4::ToggleButton {
        let button = gtk4::ToggleButton::builder()
            .icon_name(tool.get_icon_name())
            .build();
        self.tool_container_box.insert(&button, -1);
        let mut page_content = Box::new(Orientation::Vertical, 0);

        if tool.get_icon_name() == "md-tool-fill" {
            super::add_fill_tool_page(&mut page_content);
        } else if tool.get_icon_name() == "md-tool-rectangle" {
            super::add_rectangle_tool_page(&mut page_content);
        } else if tool.get_icon_name() == "md-tool-circle" {
            super::add_ellipse_tool_page(&mut page_content);
        } else if tool.get_icon_name() == "md-tool-line" {
            super::add_line_tool_page(&mut page_content);
        } else if tool.get_icon_name() == "md-tool-draw" {
            super::add_brush_tool_page(&mut page_content);
        } else if tool.get_icon_name() == "md-tool-erase" {
            super::add_erase_tool_page(&mut page_content);
        } else if tool.get_icon_name() == "md-tool-font" {
            super::add_font_tool_page(&self.window, &mut page_content);
        }

        let page_num = self.tool_notebook.append_page(&page_content, Option::<&gtk4::Widget>::None);
        let nb = &self.tool_notebook;
        button.connect_toggled(glib::clone!(@weak nb => move |_| {
            unsafe {
                crate::WORKSPACE.selected_tool = page_num as usize;
            }
            nb.set_page(page_num as i32);
        }));
        button
    }

    fn load_page(&self, buf: Buffer) -> Rc<RefCell<Editor>> {
        let child2 = AnsiView::new();
        let scroller = gtk4::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&child2)
            .build();
        let handle = Rc::new(RefCell::new(Editor::new(0, buf)));

        let page_box = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        let stack = gtk4::Stack::new();
        stack.add_child(&scroller);
        stack.add_child(&super::get_settings_page(self, handle.clone()));
    
        page_box.append(&stack);
        let caret_pos_label = gtk4::Label::new(Some(""));
        caret_pos_label.set_valign(gtk4::Align::Center);

        let mut key_preview_buf = Buffer::new();
        key_preview_buf.width = 4 * 12;
        key_preview_buf.height = 1;
        let mut key_preview_editor = Editor::new(0, key_preview_buf);
        key_preview_editor.is_inactive = true;
        let key_handle = Rc::new(RefCell::new(key_preview_editor));

        let key_set_view = AnsiView::new();
        key_set_view.set_valign(gtk4::Align::Center);
        key_set_view.set_editor_handle(key_handle.clone());
        let status_bar = gtk4::Box::new(Orientation::Horizontal, 8);
        status_bar.set_margin_start(12);
        status_bar.set_margin_end(12);
        status_bar.append(&caret_pos_label);
        status_bar.append(&gtk4::Box::builder().hexpand(true).build());

        let size_label = gtk4::Label::new(Some(""));
        size_label.set_valign(gtk4::Align::Center);
        status_bar.append(&size_label);

        status_bar.append(&gtk4::Box::builder().hexpand(true).build());
        status_bar.append(&key_set_view);

        page_box.append(&status_bar);
        
        let page = Rc::new(self.tab_view.add_page(&page_box, None));
        
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        
        gesture.connect_pressed(glib::clone!(@strong self as this => move |_, _clicks, _, _| {
            if stack.visible_child() == stack.first_child() {
                stack.set_visible_child(&stack.last_child().unwrap());
            } else {
                stack.set_visible_child(&stack.first_child().unwrap());
            }
        }));
        size_label.add_controller(&gesture);

        handle.borrow_mut().cursor.pos_changed = std::boxed::Box::new(move |e, p| {
            caret_pos_label.set_text(format!("Ln {}, Col {}", p.x + 1, p.y + 1).as_str());
            if let Some(sel) = &e.cur_selection {
                size_label.set_text(format!("{} Colums x {} Rows", sel.rectangle.size.width, sel.rectangle.size.height).as_str());
            } else {
                size_label.set_text(format!("{} Colums x {} Rows", e.buf.width, e.buf.height).as_str());
            }
        });

        let key_handle2 = key_handle;
        handle.borrow_mut().outline_changed = std::boxed::Box::new(move |editor| {
            MainWindow::update_keyset_view(editor, key_handle2.clone());
            key_set_view.queue_draw();
        });
        let handle2 = handle.clone();
        let handle3 = handle.clone();

        // force outline update.
        handle.borrow_mut().set_cur_outline(0);

        child2.set_editor_handle(handle);

        self.tab_view.set_selected_page(&page);
        child2.grab_focus();

        self.tab_to_view.borrow_mut().insert(page.clone(), Rc::new(child2));
        self.page_swap();



        MainWindow::set_file_name_for_page(&page, &handle3);
        handle2.borrow_mut().buf.file_name_changed = std::boxed::Box::new(move || {
            MainWindow::set_file_name_for_page(&page, &handle3);
        });
        handle2.borrow_mut().set_cursor_position(Position::from(0, 0));
        handle2
    }

    fn set_file_name_for_page(page: &Rc<TabPage>, editor: &Rc<RefCell<Editor>>)
    {
        if let Some(x) = &editor.borrow().buf.file_name {
            let fin = x
                .as_path()
                .file_name()
                .ok_or_else(|| panic!("Can't convert file name"))
                .unwrap();
            page.set_title(fin.to_str().unwrap());
        }  else {
            page.set_title("Untitled");
        }
    }

    fn update_keyset_view(editor: &Editor, key_handle: Rc<RefCell<Editor>>) {
        let out_buf = &mut key_handle.borrow_mut().buf;
        let mut x = 0;
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: b'S',
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: b'e',
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: b't',
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: b' ',
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;
        let outline = editor.get_cur_outline();
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: if outline > 8 { b'1' } else { b' ' },
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: b'0' + ((outline + 1) % 10) as u8,
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            Some(DosChar {
                char_code: b' ',
                attribute: TextAttribute::from_color(9, 0),
            }),
        );
        x += 1;

        for i in 0..10 {
            out_buf.set_char(0,
                Position::from(x, 0),
                Some(DosChar {
                    char_code: b' ',
                    attribute: TextAttribute::from_color(0, 4),
                }),
            );
            x += 1;

            if i == 9 {
                out_buf.set_char(0,
                    Position::from(x, 0),
                    Some(DosChar {
                        char_code: b'1',
                        attribute: TextAttribute::from_color(0, 4),
                    }),
                );
                x += 1;
                out_buf.set_char(0,
                    Position::from(x, 0),
                    Some(DosChar {
                        char_code: b'0',
                        attribute: TextAttribute::from_color(0, 4),
                    }),
                );
                x += 1;
            } else {
                out_buf.set_char(0,
                    Position::from(x, 0),
                    Some(DosChar {
                        char_code: i + b'1',
                        attribute: TextAttribute::from_color(0, 4),
                    }),
                );
                x += 1;
            }
            out_buf.set_char(0,
                Position::from(x, 0),
                Some(DosChar {
                    char_code: b'=',
                    attribute: TextAttribute::from_color(0, 4),
                }),
            );
            x += 1;
            out_buf.set_char(0,
                Position::from(x, 0),
                Some(DosChar {
                    char_code: editor.get_outline_char_code(i as i32).unwrap(),
                    attribute: TextAttribute::from_color(15, 4),
                }),
            );
            x += 1;
        }
    }
}
