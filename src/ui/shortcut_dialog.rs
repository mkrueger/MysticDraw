use gtk4::traits::{WidgetExt, GtkWindowExt};

use super::MainWindow;

static UI_SRC: &str = include_str!("shortcut_dialog.ui");

pub fn show_shortcut_dialog(main_window: &MainWindow)
{
    let builder = gtk4::Builder::from_string(UI_SRC);

    let window: gtk4::ShortcutsWindow = builder.object("shortcut-window").expect("Couldn't get window");
    window.set_transient_for(Some(&main_window.window));
    window.show();
}