use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{glib};
use std::cell::RefCell;
use std::rc::Rc;

use crate::model::Editor;

#[derive(Default)]

pub struct GtkAnsiView {
    pub editor: RefCell<Rc<RefCell<Editor>>>,
}

impl GtkAnsiView {}

#[glib::object_subclass]
impl ObjectSubclass for GtkAnsiView {
    const NAME: &'static str = "GtkCharEditorView";
    type Type = super::AnsiView;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkAnsiView {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
    }
}

impl WidgetImpl for GtkAnsiView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
    }
}

impl DrawingAreaImpl for GtkAnsiView {}
