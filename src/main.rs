extern crate gio;
extern crate gtk;

use std::env::args;
use gio::prelude::*;
use gtk::prelude::*;

const BASE_CAT_PATH: &str = "resources/images";

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("glade_config.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder
        .get_object("applicationwindow1")
        .unwrap();
    
    window.set_application(Some(app));

    // Inputs
    let message_input: gtk::Entry = builder
        .get_object("message_input")
        .unwrap();
    
    // Submit button
    let button: gtk::Button = builder
        .get_object("generate_btn")
        .unwrap();
    
    // Outputs
    let message_output: gtk::Label = builder
        .get_object("message_output")
        .unwrap();
    let is_dead_switch: gtk::Switch = builder
        .get_object("is_dead_switch")
        .unwrap();
    let image_output: gtk::Image = builder
        .get_object("image_output")
        .unwrap();
    let image_output_clone = image_output.clone();

    button.connect_clicked(move |_| {
        message_output.set_text(&format!(
            "{}\n   \\\n    \\",
            message_input
                .get_text()
                .as_str()
        ));

        let cat_type = if gtk::SwitchExt::get_active(&is_dead_switch) { 
            "cat_dead.jpg" 
        } else { 
            "cat.jpg" 
        };
        image_output_clone.set_from_file(format!("{}/{}", BASE_CAT_PATH, cat_type));
        image_output_clone.show();
    });

    window.show_all();
    image_output.hide();
}

fn main() {
    let app = gtk::Application::new(
        Some("com.yasinr.catsay_gui-glade"),
        gio::ApplicationFlags::empty()
    ).expect("Failed to initialize GTK.");

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run(&args().collect::<Vec<_>>());
}
