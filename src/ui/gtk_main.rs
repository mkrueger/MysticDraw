use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use glib::clone;
use gtk4::gio::SimpleAction;
use libadwaita as adw;

use adw::{prelude::*, TabBar, TabPage, TabView};
use adw::{ApplicationWindow, HeaderBar};
use gtk4::{Application, Box, FileChooserAction, Orientation, ResponseType};

use crate::model::{Buffer, DosChar, Editor, Position, TextAttribute, Tool, TOOLS};

use super::{CharEditorView, ColorPicker, layer_view};

pub struct MainWindow {
    pub window: ApplicationWindow,
    tab_view: TabView,
    color_picker: ColorPicker,
    tab_to_view: RefCell<HashMap<Rc<TabPage>, Rc<CharEditorView>>>,
    title: adw::WindowTitle,

    layer_listbox_model: layer_view::Model,
    layer_listbox: gtk4::ListBox
}

impl MainWindow {
    pub fn build_ui(app: &Application) {
        let content = Box::new(Orientation::Vertical, 0);
        let (title, header_bar) = MainWindow::construct_titlebar();

        let main_window = Rc::new(MainWindow {
            window: ApplicationWindow::builder()
                .application(app)
                .default_width(350)
                .content(&content)
                .build(),
            tab_view: TabView::builder().vexpand(true).build(),
            color_picker: ColorPicker::new(),
            tab_to_view: Default::default(),
            title,
            layer_listbox_model: layer_view::Model::new(),
            layer_listbox: gtk4::ListBox::new()
        });
        content.append(&header_bar);
        let tab_box = Box::new(Orientation::Vertical, 0);
        let tab_bar = TabBar::builder().view(&main_window.tab_view).build();
        tab_box.append(&tab_bar);
        tab_box.append(&main_window.tab_view);

        let right_pane = gtk4::Paned::builder()
            .orientation(Orientation::Horizontal)
            .start_child(&tab_box)
            .end_child(&main_window.construct_right_toolbar())
            .build();
        right_pane.set_position(200);

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

        main_window.layer_listbox.connect_row_selected(clone!(@strong main_window => move |_, row| {
            if let Some(row) = row {
                let idx = row.index();
                if let Some(e) = main_window.get_current_editor() {
                    e.borrow_mut().cur_layer = idx;
                }
            }   
        }));

        main_window.layer_listbox.set_activate_on_single_click(false);
        main_window.layer_listbox.connect_row_activated(clone!(@strong main_window => move |_, row| {
            let idx = row.index();
            if let Some(e) = main_window.get_current_editor() {
                let res = Rc::new(layer_view::display_edit_layer_dialog(&main_window.window, &e.borrow_mut().buf.layers[idx as usize]));
                let rd = &res.clone().dialog;
                rd.connect_response(clone!(@strong main_window => move |dialog, r| {
                    if let ResponseType::Ok = r {
                        res.set_layer_values(&mut e.borrow_mut().buf.layers[idx as usize])
                    } 
                    dialog.close();
                    main_window.update_layer_view();
                    main_window.update_editor();
                }));
            }
        }));

        {
            //  let rc = rc.clone();
            let open_action = SimpleAction::new("new", None);
            open_action.connect_activate(clone!(@strong main_window => move |_,_| {
                let mut buffer = Buffer::new();
                buffer.file_name = None;
                buffer.width  = 80;
                buffer.height = 25;
                main_window.load_page(buffer);
                main_window.update_layer_view();
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
                    .width_request(640)
                    .height_request(480)
                    .build();

                file_chooser.add_button("Open", ResponseType::Ok);
                file_chooser.add_button("Cancel", ResponseType::Cancel);

                file_chooser.connect_response(clone!(@strong main_window => move |d, response| {
                    if response == ResponseType::Ok {
                        let file = d.file().expect("Couldn't get file");
                        let filename = file.path().expect("Couldn't get file path");
                        let buffer = Buffer::load_buffer(filename.as_path());
                        if let Ok(buf) = buffer {
                            main_window.load_page(buf);
                            std::env::set_current_dir(filename.parent().unwrap()).expect("can't set current path.");
                        }
                    }
                    d.close();
                }));
                file_chooser.show();

            }));
            app.add_action(&open_action);
        }

        {
            let open_action = SimpleAction::new("save", None);
            open_action.connect_activate(clone!(@strong main_window => move |_,_| {
                let file_chooser = gtk4::FileChooserDialog::builder()
                    .title("Save file")
                    .action(FileChooserAction::Save)
                    .transient_for(&main_window.window)
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
                            page.get_editor().borrow().save_content(&filename);
                            println!("{}", filename.to_str().unwrap());
                            page.get_editor().borrow_mut().buf.file_name = Some(filename);
                            println!("file changed !!! {:?}",page.get_editor().borrow().buf.file_name); 
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
                    new_layer.name = "New layer".to_string();
                    editor.borrow_mut().buf.layers.insert(0, new_layer);
                    main_window.update_layer_view();
                }
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
                            main_window.get_current_ansi_view().unwrap().queue_draw();
                        }
                    }
                }
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
                            main_window.get_current_ansi_view().unwrap().queue_draw();
                        }
                    }
                }
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
                    main_window.get_current_ansi_view().unwrap().queue_draw();
                }
            }));
            app.add_action(&action);
        }
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
        }
        self.update_layer_view();
    }

    pub fn update_layer_view(&self)
    {
        self.layer_listbox_model.clear();
        if let Some(editor) = self.get_current_editor() {
            for b in &editor.borrow().buf.layers {
                self.layer_listbox_model.append(&layer_view::RowData::new(&b.name, b.is_visible));
            }
            let len = editor.borrow().buf.layers.len();
            if len > 0 {
                let row = self.layer_listbox.row_at_index(len as i32 - 1);
                if let Some(row)= row {
                    self.layer_listbox.select_row(Some(&row));
                }
            }
        }
    }

    pub fn update_editor(&self)
    {
        self.get_current_ansi_view().unwrap().queue_draw();
    }

    pub fn get_current_editor(&self) -> Option<Rc<RefCell<Editor>>> {
        let cur = self.get_current_ansi_view();
        cur.map(|view| view.get_editor())
    }

    pub fn get_current_ansi_view(&self) -> Option<Rc<CharEditorView>> {
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

        hb.pack_end(
            &gtk4::MenuButton::builder()
                .icon_name("open-menu-symbolic")
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

        result.append(&self.color_picker);

        let flow_box = gtk4::FlowBox::builder()
            .valign(gtk4::Align::Start)
            .selection_mode(gtk4::SelectionMode::None)
            .build();

        let nb = gtk4::Notebook::builder().show_tabs(false).build();

        unsafe {
            let first = self.add_tool(&flow_box, &nb, TOOLS[0]);
            for t in TOOLS.iter().skip(1) {
                self.add_tool(&flow_box, &nb, *t).set_group(Some(&first));
            }
        }

        nb.set_page(0);
        result.append(&flow_box);
        result.append(&nb);
        result
    }

    fn construct_right_toolbar(&self) -> Box {
        let result = Box::new(Orientation::Vertical, 0);

        let stack = gtk4::Stack::new();
        
        let page = stack.add_child(&self.construct_layer_view());
        page.set_name("page1");
        page.set_title("Layer");

        let page = stack.add_child(&self.construct_channels());
        page.set_name("page2");
        page.set_title("Channels");

        result.append(&stack);

        result
    }

    fn construct_layer_view(&self) -> gtk4::Box {
        let result = Box::new(Orientation::Vertical, 0);
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
            .min_content_height(480)
            .min_content_width(360)
            .vexpand(true)
            .build();
        
        scrolled_window.set_child(Some(&self.layer_listbox));
        result.append(&scrolled_window);

        let toolbar = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .vexpand(false)
        .build();
        toolbar.style_context().add_class("toolbar");
        let new_layer_button = gtk4::Button::builder()
            .icon_name("document-new")
            .action_name("app.layer-new")
            .build();
        toolbar.append(&new_layer_button);

        let layer_up_button = gtk4::Button::builder()
            .icon_name("go-up")
            .action_name("app.layer-up")
            .build();
        toolbar.append(&layer_up_button);

        let layer_down_button = gtk4::Button::builder()
            .icon_name("go-down")
            .action_name("app.layer-down")
            .build();
        toolbar.append(&layer_down_button);

        let layer_delete_button = gtk4::Button::builder()
            .icon_name("edit-delete")
            .action_name("app.layer-delete")
            .build();
        toolbar.append(&layer_delete_button);

        result.append(&toolbar);
        result
    }

    fn construct_channels(&self) -> Box {
        let result = Box::new(Orientation::Vertical, 0);
        let fg_button = gtk4::CheckButton::builder().label("Foreground").build();
        result.append(&fg_button);

        let bg_button = gtk4::CheckButton::builder().label("Background").build();
        result.append(&bg_button);
        result
    }

    fn add_tool(
        &self,
        flow_box: &gtk4::FlowBox,
        nb: &gtk4::Notebook,
        tool: &dyn Tool,
    ) -> gtk4::ToggleButton {
        let button = gtk4::ToggleButton::builder()
            .icon_name(tool.get_icon_name())
            .build();
        flow_box.insert(&button, -1);
        let page_content = Box::new(Orientation::Vertical, 0);
        // tool.add_tool_page(window, &mut page_content);

        let page_num = nb.append_page(&page_content, Option::<&gtk4::Widget>::None);

        button.connect_toggled(glib::clone!(@weak nb => move |_| {
            unsafe {
                crate::WORKSPACE.selected_tool = page_num as usize;
            }
            nb.set_page(page_num as i32);
        }));
        button
    }

    fn load_page(&self, buf: Buffer) {
        let child2 = CharEditorView::new();
        let scroller = gtk4::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .child(&child2)
            .build();

        let page_box = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
            .build();

        page_box.append(&scroller);
        let caret_pos_label = gtk4::Label::new(Some("( 1, 1)"));

        let mut key_preview_buf = Buffer::new();
        key_preview_buf.width = 4 * 12;
        key_preview_buf.height = 1;
        let mut key_preview_editor = Editor::new(0, key_preview_buf);
        key_preview_editor.is_inactive = true;
        let key_handle = Rc::new(RefCell::new(key_preview_editor));

        let key_set_view = CharEditorView::new();
        key_set_view.set_editor_handle(key_handle.clone());
        let status_bar = gtk4::Box::new(Orientation::Horizontal, 8);
        status_bar.append(&caret_pos_label);
        status_bar.append(&gtk4::Box::builder().hexpand(true).build());
        status_bar.append(&key_set_view);

        page_box.append(&status_bar);

        let page = Rc::new(self.tab_view.add_page(&page_box, None));
        let handle = Rc::new(RefCell::new(Editor::new(0, buf)));

        handle.borrow_mut().cursor.changed = std::boxed::Box::new(move |p| {
            caret_pos_label.set_text(format!("({:>2},{:>3})", p.x + 1, p.y + 1).as_str());
        });

        let key_handle2 = key_handle.clone();
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
            DosChar {
                char_code: b'S',
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            DosChar {
                char_code: b'e',
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            DosChar {
                char_code: b't',
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            DosChar {
                char_code: b' ',
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;
        let outline = editor.get_cur_outline();
        out_buf.set_char(0,
            Position::from(x, 0),
            DosChar {
                char_code: if outline > 8 { b'1' } else { b' ' },
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            DosChar {
                char_code: b'0' + ((outline + 1) % 10) as u8,
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;
        out_buf.set_char(0,
            Position::from(x, 0),
            DosChar {
                char_code: b' ',
                attribute: TextAttribute::from_color(9, 0),
            },
        );
        x += 1;

        for i in 0..10 {
            out_buf.set_char(0,
                Position::from(x, 0),
                DosChar {
                    char_code: b' ',
                    attribute: TextAttribute::from_color(0, 4),
                },
            );
            x += 1;

            if i == 9 {
                out_buf.set_char(0,
                    Position::from(x, 0),
                    DosChar {
                        char_code: b'1',
                        attribute: TextAttribute::from_color(0, 4),
                    },
                );
                x += 1;
                out_buf.set_char(0,
                    Position::from(x, 0),
                    DosChar {
                        char_code: b'0',
                        attribute: TextAttribute::from_color(0, 4),
                    },
                );
                x += 1;
            } else {
                out_buf.set_char(0,
                    Position::from(x, 0),
                    DosChar {
                        char_code: i + b'1',
                        attribute: TextAttribute::from_color(0, 4),
                    },
                );
                x += 1;
            }
            out_buf.set_char(0,
                Position::from(x, 0),
                DosChar {
                    char_code: b'=',
                    attribute: TextAttribute::from_color(0, 4),
                },
            );
            x += 1;
            out_buf.set_char(0,
                Position::from(x, 0),
                DosChar {
                    char_code: editor.get_outline_char_code(i as i32).unwrap(),
                    attribute: TextAttribute::from_color(15, 4),
                },
            );
            x += 1;
        }
    }
}
