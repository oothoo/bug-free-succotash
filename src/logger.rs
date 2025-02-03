use std::fs::{OpenOptions};
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub fn log(&self, message: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("delta_executor.log")
            .expect("Unable to open log file");

        writeln!(file, "{}", message).expect("Unable to write to log file");
    }
}
