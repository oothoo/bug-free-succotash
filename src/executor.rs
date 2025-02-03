use std::process::{Command, Stdio};

pub struct Executor {
    process: Option<std::process::Child>,
}

impl Executor {
    pub fn new() -> Self {
        Executor { process: None }
    }

    pub fn run_script(&mut self, script: String) {
        let mut command = Command::new("roblox-player");
        command.args(&["--script", &script]);

        let process = command.spawn().expect("Failed to start Roblox process");
        self.process = Some(process);
    }

    pub fn stop(&mut self) {
        if let Some(mut process) = self.process.take() {
            process.kill().expect("Failed to stop process");
        }
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }
}
