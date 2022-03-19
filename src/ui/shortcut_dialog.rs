use gtk4::traits::{BoxExt, WidgetExt};

pub fn show_shortcut_dialog()
{
    let general_group = gtk4::ShortcutsGroup::builder()
        .title("General")
        .build();
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Undo")
        .accelerator("<primary>z")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Redo")
        .accelerator("<primary><Shift>z")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Select all")
        .accelerator("<primary>a")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Unselect")
        .accelerator("Escape")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Move to next tab")
        .accelerator("Tab")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Move to prev tab")
        .accelerator("<Shift>Tab")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Cut")
        .accelerator("<primary>x")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Copy")
        .accelerator("<primary>c")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Paste")
        .accelerator("<primary>v")
        .build());

    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Left justify")
        .accelerator("<Alt>l")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Right justify")
        .accelerator("<Alt>r")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Center")
        .accelerator("<Alt>r")
        .build());
    general_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Draw function key")
        .accelerator("F1F12")
        .build());

    let color_group = gtk4::ShortcutsGroup::builder()
        .title("Colors")
        .build();

    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Prev foreground color")
        .accelerator("<primary>Up")
        .build());

    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Next foreground color")
        .accelerator("<primary>Down")
        .build());
    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Prev background color")
        .accelerator("<primary>Left")
        .build());

    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Next background color")
        .accelerator("<primary>Right")
        .build());

    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Pickup attribute under cursor")
        .accelerator("<Alt>u")
        .build());
    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Default color")
        .accelerator("<primary>d")
        .build());

    color_group.append(&gtk4::ShortcutsShortcut::builder()
        .title("Switch fore and background")
        .accelerator("<primary><shift>x")
        .build());

    let section = gtk4::ShortcutsSection::builder()
        .name("shortcuts")
        .max_height(10)
        .build();
    
    section.append(&general_group);
    section.append(&color_group);

    let window = gtk4::ShortcutsWindow::builder()
        .modal(true)
        .child(&section)
      /*   .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)*/
        .build();
    window.show();
}