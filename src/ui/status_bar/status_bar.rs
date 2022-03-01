use glib::subclass::{types::ObjectSubclass, object::ObjectImpl};
use gtk4::{subclass::prelude::{WidgetImpl, WidgetImplExt, BoxImpl}, Label};


// Object holding the state
pub struct GtkAnsiStatusBar {
  label: Label
}

impl Default for GtkAnsiStatusBar {
    fn default() -> Self {
        Self {
          label: Label::new(None)
        }
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for GtkAnsiStatusBar {
    const NAME: &'static str = "MyStatusBar";
    type Type = super::AnsiStatusBar;
    type ParentType = gtk4::Box;
}

impl ObjectImpl for GtkAnsiStatusBar {

}

impl WidgetImpl for GtkAnsiStatusBar {
  fn realize(&self, widget: &Self::Type) {
    self.parent_realize(widget);
  }
}

impl BoxImpl for GtkAnsiStatusBar {
  
  /* 
  fn realize(&self, widget: &Self::Type) {
      self.parent_realize(widget);

      let statusLabel = Label::new(None);
    }*/
}
