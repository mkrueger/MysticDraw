mod gtkchar_editor_view;
mod char_renderer;


use glib::subclass::types::ObjectSubclassIsExt;
use gtk4::{ glib, traits::{ GLAreaExt, WidgetExt}, gdk};

use self::gtkchar_editor_view::GtkCharEditorView;


glib::wrapper! {
    pub struct CharEditorView(ObjectSubclass<GtkCharEditorView>) @extends gtk4::GLArea, gtk4::Widget;
}

impl Default for CharEditorView {
    fn default() -> Self {
        Self::new()
    }
}

impl CharEditorView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a AnsiEditorArea")
    }

    pub fn set_buffer(&self, buffer_id: usize)
    {
        let imp = self.imp();
        imp.buf.set(buffer_id);
        
        unsafe {
            let buffer = &crate::buffer::ALL_BUFFERS[buffer_id];
            self.set_size_request(buffer.base_layer.width as i32 * 8, buffer.base_layer.height as i32 * 16);
        }

        if imp.renderer.borrow_mut().as_ref().is_some() {
            imp.renderer.borrow_mut().as_ref().unwrap().buffer_id.set(buffer_id);
        }

    }
}

unsafe impl glium::backend::Backend for CharEditorView {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        // We're supposed to draw (and hence swap buffers) only inside the `render()` vfunc or
        // signal, which means that GLArea will handle buffer swaps for us.
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::ffi::c_void {
        epoxy::get_proc_addr(symbol)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let scale = self.scale_factor();
        let width = self.width();
        let height = self.height();
        ((width * scale) as u32, (height * scale) as u32)
    }

    fn is_current(&self) -> bool {
        match self.context() {
            Some(context) => gdk::GLContext::current() == Some(context),
            None => false,
        }
    }

    unsafe fn make_current(&self) {
        GLAreaExt::make_current(self);
    }
}