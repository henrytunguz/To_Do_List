````markdown name=README.md
# To Do List

A simple To-Do List application written in Rust. This project leverages the `eframe` and `egui` libraries for a native GUI, and supports features like date picking and persistent task management using `serde` and `serde_json`.

## Features

- Add, remove, and mark tasks as completed
- Persistent storage of tasks in JSON format
- Date selection for tasks with a date picker (via `egui_extras`)
- Simple, native GUI

## Dependencies

- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe): for GUI application framework
- [egui_extras](https://github.com/emilk/egui/tree/master/crates/egui_extras): provides additional GUI widgets like date picker
- [chrono](https://github.com/chronotope/chrono): date and time handling
- [serde](https://github.com/serde-rs/serde) and [serde_json](https://github.com/serde-rs/json): serialization/deserialization for saving/loading tasks
- [env_logger](https://github.com/env-logger-rs/env_logger): logging

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2021 or later)

### Build and Run

Clone the repository and build the project with Cargo:

```bash
git clone https://github.com/henrytunguz/To_Do_List.git
cd To_Do_List
cargo run --release
```

This will start the GUI To-Do List application.

## Usage

- Add new tasks using the input field
- Assign due dates using the date picker
- Mark tasks as completed or remove them as needed
- All tasks are saved between sessions

## Project Structure

- `src/main.rs`: Main application code and GUI logic
- `Cargo.toml`: Project metadata and dependencies

## License

This project is licensed under the MIT License.

## Acknowledgements

- Built with [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) and [egui](https://github.com/emilk/egui)
````
