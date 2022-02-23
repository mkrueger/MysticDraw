use gtk4::glib;
use gtk4::subclass::prelude::*;
use gtk4::traits::GLAreaExt;
use gtk4::traits::WidgetExt;
use std::cell::Cell;
use std::cell::RefCell;


use super::char_renderer::CharRenderer;

#[derive(Default)]
pub struct GtkCharEditorView {
    pub renderer: RefCell<Option<CharRenderer>>,
    pub buf: Cell<usize>
}

impl GtkCharEditorView
{
    pub fn set_editor(&self, editor_id: usize)
    {
        let opt = &*self.renderer.borrow();
        if let Some(r) = opt {
            r.editor_id.set(editor_id);
        } else {
            self.buf.set(editor_id);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkCharEditorView {
    const NAME: &'static str = "AnsiEditorArea";
    type Type = super::CharEditorView;
    type ParentType = gtk4::GLArea;
}

impl ObjectImpl for GtkCharEditorView {

    fn constructed(&self, obj: &Self::Type) {
        obj.set_can_focus(true);
        obj.set_focusable(true);
        obj.set_focus_on_click(true);
        
        let gesture = gtk4::GestureClick::new();
        // Trigger a transition on click
        gesture.connect_pressed(glib::clone!(@strong obj as this => move |_, clicks, x, y| {
            println!("gesture click {}, {}, {}", clicks ,x, y);
        }));
        obj.add_controller(&gesture);
        let id = self.buf.get();
        let key = gtk4::EventControllerKey::new();
        key.connect_key_pressed(glib::clone!(@strong obj as this => move |_, key, key_code, modifier| {
            {
                crate::Workspace::get_editor(id).handle_key(key, key_code, modifier);
            }
            glib::signal::Inhibit(true)
        }));
        obj.add_controller(&key);

    }
}

impl WidgetImpl for GtkCharEditorView {
    fn realize(&self, widget: &Self::Type) {
        self.parent_realize(widget);

        if widget.error().is_some() {
            return;
        }
        let context =
            unsafe { glium::backend::Context::new(self.instance(), true, Default::default()) }
                .unwrap();
        *self.renderer.borrow_mut() = Some(CharRenderer::new(self.buf.get(), context));
    }

    fn unrealize(&self, widget: &Self::Type) {
        *self.renderer.borrow_mut() = None;

        self.parent_unrealize(widget);
    }
}

impl GLAreaImpl for GtkCharEditorView {
    fn render(&self, _gl_area: &Self::Type, _context: &gtk4::gdk::GLContext) -> bool {
        self.renderer.borrow().as_ref().unwrap().draw();
        true
    }

    fn resize(&self, _gl_area: &Self::Type, width: i32, height: i32) {
        self.renderer.borrow().as_ref().unwrap().iresize(width, height);
    }
}