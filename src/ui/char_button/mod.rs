
use gtk4::{
    glib,
};

mod char_selector_dialog;
mod gtkchar_button;

glib::wrapper! {
    pub struct CharButton(ObjectSubclass<gtkchar_button::GtkCharButton>) @extends gtk4::Widget, gtk4::Button;
}

impl CharButton {
    pub fn new(char_code: u8) -> Self {
        let o = glib::Object::new(&[("charcode", &char_code)]).expect("Failed to create a AnsiEditorArea");

       // let x = o.imp();
       // x.main_window.replace(main_window.borrow());
        o
    }
/* 
    fn constructed(&self, obj: &Self::Type) {
       
    }*/
}
