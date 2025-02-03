mod executor;
mod gui;
mod script;
mod logger;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use executor::Executor;
use gui::create_gui;
use script::ScriptManager;
use logger::Logger;

fn main() {
    let app = Application::new(Some("com.delta.deltaexecutor"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Delta Executor");
        window.set_default_size(800, 600);

        let executor = Executor::new();
        let script_manager = ScriptManager::new();
        let logger = Logger::new();

        let gui = create_gui(&window, &executor, &script_manager, &logger);
        window.add(&gui);
        window.show_all();
    });

    app.run();
}
