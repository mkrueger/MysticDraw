use gtk4::glib;
use gtk4::subclass::prelude::*;
use gtk4::traits::GLAreaExt;
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
    pub fn set_buffer(&self, buffer_id: usize)
    {
        let opt = &*self.renderer.borrow();
        if let Some(r) = opt {
            r.buffer_id.set(buffer_id);
        } else {
            self.buf.set(buffer_id);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for GtkCharEditorView {
    const NAME: &'static str = "AnsiEditorArea";
    type Type = super::CharEditorView;
    type ParentType = gtk4::GLArea;
}

impl ObjectImpl for GtkCharEditorView {}

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