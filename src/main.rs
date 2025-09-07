use gtk4::prelude::*;
use gtk4::Application;

mod models;
mod services;
mod ui;

use ui::build_main_window;

fn main() {
    let app = Application::builder()
        .application_id("com.waifugenerator.app")
        .build();

    app.connect_activate(build_main_window);

    app.run();
}
