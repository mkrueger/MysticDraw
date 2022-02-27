
use std::{rc::Rc, cell::RefCell};

use druid::{Widget, WindowDesc, LocalizedString, AppLauncher, Data, Env, WindowId, Menu, widget::{Flex, Tabs, TabsPolicy, TabInfo, Label, Axis, TabsEdge, TabsTransition}, WidgetExt };

use crate::model::{Buffer, Editor};

mod ansi_widget;
mod app_delegate;

#[derive(Debug, Clone, Default)]
struct AppState {
    pub editor: Vec<Rc<RefCell<Editor>>>,
    pub cur_tool: usize,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        if self.editor.len() != other.editor.len() || self.cur_tool != other.cur_tool {
            return false;
        }

        for i in 0..self.editor.len() {
            let e1 = self.editor[i].borrow();
            let e2 = other.editor[i].borrow();
            if e1.cursor != e2.cursor {
                return false;
            }
        }
        true
    }
}

fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = druid::platform_menus::mac::menu_bar();
    }
    let file_menu = Menu::new(LocalizedString::new("common-menu-file-menu"))
        .entry(druid::platform_menus::win::file::new())
        .entry(druid::platform_menus::win::file::open())
        .entry(druid::platform_menus::win::file::close())
        .entry(druid::platform_menus::win::file::save_ellipsis())
        .entry(druid::platform_menus::win::file::save_as())
        .separator()
        .entry(druid::platform_menus::win::file::exit());
    base = base.entry(file_menu);
    base
}

#[derive(Clone, Data)]
struct NumberedTabs;

impl TabsPolicy for NumberedTabs {
    type Key = usize;
    type Build = ();
    type Input = AppState;
    type LabelWidget = Label<AppState>;
    type BodyWidget = druid::widget::Scroll<AppState, Flex<AppState>>;

    fn tabs_changed(&self, old_data: &AppState, data: &AppState) -> bool {
        old_data.editor.len() != data.editor.len()
    }

    fn tabs(&self, data: &AppState) -> Vec<Self::Key> {
        (0..data.editor.len()).collect()
    }

    fn tab_info(&self, key: Self::Key, _data: &AppState) -> TabInfo<AppState> {
        TabInfo::new(format!("Editor {}", key), true)
    }

    fn tab_body(&self, key: Self::Key, _data: &AppState) -> Self::BodyWidget {
        let mut widget = ansi_widget::AnsiWidget::new(_data.editor[key].clone());
        widget.initialize();

        let mut col = Flex::column();
        col.add_child(widget);
        col.scroll()
    }

    fn close_tab(&self, key: Self::Key, data: &mut AppState) {
      //  if let Some(idx) = data.tab_labels.index_of(&key) {
            data.editor.remove(key);
      //  }
    }

    fn tab_label(
        &self,
        _key: Self::Key,
        info: TabInfo<Self::Input>,
        _data: &Self::Input,
    ) -> Self::LabelWidget {
        Self::default_make_label(info)
    }
}

fn build_widget() -> impl Widget<AppState> {
    let dyn_tabs = Tabs::for_policy(NumberedTabs)
    .with_axis(Axis::Horizontal)
    .with_edge(TabsEdge::Leading)
    .with_transition(TabsTransition::Instant)
    ;
    dyn_tabs
}

pub fn start_druid_app() {
    let window = WindowDesc::new(build_widget())
    .title(LocalizedString::new("Mystic Draw"))
    .menu(make_menu);

    let mut state = AppState {
        editor: Vec::new(),
        cur_tool: 0
    };

    let buffer = Buffer::load_buffer(std::path::Path::new("/home/mkrueger/Downloads/test.xb")).unwrap();
    let editor = crate::model::Editor::new(0, buffer);
    state.editor.push(Rc::new(RefCell::new(editor)));

    AppLauncher::with_window(window)
        .delegate(app_delegate::Delegate {})
        .log_to_console()
        .launch(state)
        .expect("launch failed");
}

/* // for testing purposes
fn build_widget2() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let buffer = Buffer::load_buffer(std::path::Path::new("test.xb")).unwrap();
    let editor = crate::model::Editor::new(0, buffer);
    let mut widget = ansi_widget::AnsiWidget::new(Rc::new(editor));
    widget.initialize();
    col.add_child( widget);
    col.scroll()
}*/
