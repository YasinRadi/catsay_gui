extern crate gio;
extern crate gtk;

use std::env;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Image,
    Label,
    Orientation,
    Application, 
    ApplicationWindow
};

const CAT_IMG_PATH: &str = "images/cat.jpg";

fn main() {
    let app = Application::new(
        Some("com.yasinr.catsay_gui"),
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK.");

    // Application set up
    app.connect_startup(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 70);

        window.connect_delete_event(|win, _| {
            unsafe {
                win.destroy();
            }
            Inhibit(false)
        });

        let layout_box = gtk::Box::new(Orientation::Vertical, 0);
        let label = Label::new(Some("Meow!\n     \\\n       \\"));
        
        layout_box.add(&label);

        let cat_image = Image::from_file(CAT_IMG_PATH);

        layout_box.add(&cat_image);

        window.add(&layout_box);
        window.show_all();
    });

    app.connect_activate(|_| {});
    app.run(&env::args().collect::<Vec<_>>());
}
