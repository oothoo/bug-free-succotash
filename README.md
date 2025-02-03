> [!IMPORTANT]
> Delta Executor is provided for educational purposes only. Do not use it to disrupt games or harm other players. This project is not affiliated with Roblox Corporation.

# Delta Executor
Delta Executor is a Roblox executor (a script executor for Roblox games), built in Rust. It allows you to run custom scripts in Roblox using a graphical interface. Designed for Windows, it operates without being detected by anti-cheat systems. This project is maintained by [oothoo](https://github.com/oothoo).

## Download
1. Go to the [Releases](https://github.com/oothoo/bug-free-succotash/releases) section of this repository.
2. Download the latest version.
3. Unzip the downloaded file.
4. Run `DeltaExecutor.exe` (no installation required).

## Usage
1. Launch Roblox and join a game.
2. Open Delta Executor.
3. Use the `Inject` button to connect to the running Roblox process.
4. Write or paste scripts into the editor, then press `Execute` to run them.
5. Scripts will run in the game immediately.

## Features
- Undetectable: Works without triggering Roblox anti-cheat systems.
- Graphical interface: No command-line required.
- Built-in script editor: Write or paste scripts directly into the app.
- Syntax highlighting: Auto-formats Lua code for readability.
- Multi-threaded execution: Run scripts without freezing the game.
- Auto-update: Get notified when new versions are available.
- Process scanner: Automatically detects Roblox processes.
- File system access: Save and load scripts from your computer.
- Custom keybinds: Assign shortcuts to execute scripts.
- Error logging: Displays detailed messages if a script fails.

## Build from Source
To compile Delta Executor yourself:
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository:  
   ```
   git clone https://github.com/oothoo/bug-free-succotash.git
   ```
3. Navigate to the project folder and build:  
   ```
   cargo build --release
   ```
   The executable will be in `target/release/`.

## Contributing
Contributions are currently not accepted. The development team is too small to review external code or manage pull requests effectively.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.