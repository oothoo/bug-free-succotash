use gtk::prelude::*;
use gtk::{Button, TextView, ScrolledWindow, Box};
use executor::Executor;
use script::ScriptManager;
use logger::Logger;

pub fn create_gui(
    window: &gtk::ApplicationWindow,
    executor: &Executor,
    script_manager: &ScriptManager,
    logger: &Logger,
) -> Box {
    let vbox = Box::new(gtk::Orientation::Vertical, 10);

    let run_button = Button::with_label("Run Script");
    run_button.connect_clicked(move |_| {
        let script = script_manager.load_script("example.lua");
        executor.run_script(script);
        logger.log("Script started");
    });

    let stop_button = Button::with_label("Stop Script");
    stop_button.connect_clicked(move |_| {
        executor.stop();
        logger.log("Script stopped");
    });

    let log_window = ScrolledWindow::new(None, None);
    let log_view = TextView::new();
    log_window.add(&log_view);

    vbox.pack_start(&run_button, false, false, 5);
    vbox.pack_start(&stop_button, false, false, 5);
    vbox.pack_start(&log_window, true, true, 5);

    vbox
}
