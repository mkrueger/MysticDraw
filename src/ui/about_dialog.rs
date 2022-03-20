use gtk4::traits::WidgetExt;

use super::MainWindow;

pub fn show_about_dialog(main_window: &MainWindow)
{
    let about_dialog = gtk4::AboutDialog::builder()
        .title("About Mystic Draw")
        .comments("ANSI screen editor, written in Rust\nContains algorithms from Pablo Draw and Moebius.")
        .copyright("\u{a9} 2022 Mike Krüger.\nIcons from Microsoft fluent\nAll screen fonts are property of the copyright owners.")
        .license_type(gtk4::License::Apache20)
        .program_name("Mystic Draw")
        .website("https://github.com/mkrueger/MysticDraw")
        .logo_icon_name("about")
        .modal(true)
        .transient_for(&main_window.window)
        .build();
    about_dialog.show();
}