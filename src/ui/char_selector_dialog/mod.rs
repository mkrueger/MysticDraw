use libadwaita::ApplicationWindow;


mod imp {
    use gtk4::{ glib, subclass::prelude::*};
    use libadwaita::{subclass::{prelude::*}};

    pub struct CharSelectorDialog {
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CharSelectorDialog {
        const NAME: &'static str = "CharSelectorDialog";
        type Type = super::CharSelectorDialog;
        type ParentType = libadwaita::Window;

        fn type_init(_type_: &mut glib::subclass::InitializingType<Self>) {}

        fn class_init(_klass: &mut Self::Class) {}

        fn new() -> Self {
            CharSelectorDialog { }
        }

        fn with_class(_klass: &Self::Class) -> Self {
            Self::new()
        }

        fn instance_init(_obj: &glib::subclass::InitializingObject<Self>) {}
    }
    
    impl ObjectImpl for CharSelectorDialog {
        /*   fn signals() -> &'static [Signal] {
            use once_cell::sync::Lazy;
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("changed", &[], <()>::static_type().into()).build()]
            });
            SIGNALS.as_ref()
        } */
    }
    impl WidgetImpl for CharSelectorDialog {}
    impl WindowImpl for CharSelectorDialog {}
    impl AdwWindowImpl for CharSelectorDialog {}
}


glib::wrapper! {
    pub struct CharSelectorDialog(ObjectSubclass<imp::CharSelectorDialog>) @extends gtk4::Widget, gtk4::Window, libadwaita::Window;
}

impl CharSelectorDialog {
    pub fn new(_model: ApplicationWindow) -> Self {
        let dialog =
            glib::Object::new::<CharSelectorDialog>(&[]).expect("Failed to create ProvidersDialog");
        dialog
    }
}
