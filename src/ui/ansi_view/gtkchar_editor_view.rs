use gtk4::subclass::prelude::*;
use gtk4::traits::WidgetExt;
use gtk4::{gdk, glib};
use std::cell::RefCell;
use std::rc::Rc;

use crate::model::Editor;

#[derive(Default)]

pub struct GtkCharEditorView {
    pub editor: RefCell<Rc<RefCell<Editor>>>,
}

impl GtkCharEditorView {}

#[glib::object_subclass]
impl ObjectSubclass for GtkCharEditorView {
    const NAME: &'static str = "GtkCharEditorView";
    type Type = super::CharEditorView;
    type ParentType = gtk4::DrawingArea;
}

impl ObjectImpl for GtkCharEditorView {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
    }
}

impl WidgetImpl for GtkCharEditorView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);
    }
}

impl DrawingAreaImpl for GtkCharEditorView {}
