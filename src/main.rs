use chrono::{Datelike, Local, NaiveDate, NaiveTime, Timelike};
use eframe::{egui, epaint};
use egui_extras;
use egui::{containers::panel, Ui};
use egui_extras::DatePickerButton;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
struct Task {
    title: String,
    details: String,
    #[serde(with = "color32_def")]
    color: egui::Color32,
    #[serde(with = "naive_date_def")]
    date: NaiveDate,
    #[serde(with = "naive_time_def")]
    time: NaiveTime,
    completed: bool,
}
mod naive_date_def {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

// For NaiveTime
mod naive_time_def {
    use chrono::NaiveTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&time.format("%H:%M:%S").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, "%H:%M:%S").map_err(serde::de::Error::custom)
    }
}
// Helper module for Color32 serialization as u32
mod color32_def {
    use serde::{Deserializer, Serializer, Deserialize};
    use eframe::egui::Color32;

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        // Convert RGBA to u32: (A << 24) | (B << 16) | (G << 8) | R
        let [r, g, b, a] = color.to_array();
        let value = (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | (r as u32);
        serializer.serialize_u32(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where D: Deserializer<'de> {
        let value = u32::deserialize(deserializer)?;
        let r = (value & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = ((value >> 16) & 0xFF) as u8;
        let a = ((value >> 24) & 0xFF) as u8;
        Ok(Color32::from_rgba_premultiplied(r, g, b, a))
    }
}

impl Task {
    fn new(
        title: &str,
        details: &str,
        color: egui::Color32,
        date: NaiveDate,
        time: NaiveTime,
        completed: bool,
    ) -> Self {
        Task {
            title: title.to_string(),
            details: details.to_string(),
            color,
            date,
            time,

            completed: false,
        }
    }

    fn printInfo(&self) {
        println!("Title: {}", self.title);
        println!("Details: {}", self.details);
        println!("Color: {:?}", self.color);
        println!("Date: {}", self.date);
        println!("Time: {}", self.time);
        println!("Completed: {}\n", self.completed);
    }
}

struct RectWithText{
    rect: egui::Rect,
    text: String,
}
struct TaskList{
    list: Vec<Task>,
}
impl TaskList {
    fn new(tasks: Vec<Task>) -> Self {
        TaskList { list: tasks }
    }
    fn add_and_sort(&mut self, task: Task) {
        self.list.push(task);
        self.sort_by_status_and_magnitude()

    }

    fn add_task(&mut self, task: Task) {
        self.list.push(task);

    }
    fn sort_by_status_and_magnitude(&mut self) {
        // Get current date and time
        let now = Local::now();
        let current_date = NaiveDate::from_ymd_opt(
            now.year(),
            now.month(),
            now.day()
        ).unwrap();
        let current_time = NaiveTime::from_hms_opt(
            now.hour(),
            now.minute(),
            now.second()
        ).unwrap();

        self.list.sort_by(|a, b| {
            // First, check if tasks are overdue (passed)
            let a_datetime = (a.date, a.time);
            let b_datetime = (b.date, b.time);
            let current_datetime = (current_date, current_time);

            let a_is_overdue = a_datetime < current_datetime;
            let b_is_overdue = b_datetime < current_datetime;

            // Sort by overdue status first (overdue tasks come first)
            if a_is_overdue && !b_is_overdue {
                return std::cmp::Ordering::Less;
            } else if !a_is_overdue && b_is_overdue {
                return std::cmp::Ordering::Greater;
            }

            // If both are overdue or both are not overdue, sort by magnitude (time difference)
            // For overdue tasks: the longer overdue comes first
            // For upcoming tasks: the sooner comes first
            if a_is_overdue && b_is_overdue {
                // For overdue tasks, compare in reverse (more overdue comes first)
                return b_datetime.cmp(&a_datetime);
            } else {
                // For upcoming tasks, compare normally (sooner comes first)
                return a_datetime.cmp(&b_datetime);
            }
        });
    }
    fn len(&self) -> usize {
        self.list.len()
    }

    fn remove_task(&mut self, task_to_remove: &Task) -> Option<Task> {
        if let Some(pos) = self.list.iter().position(|task| task.title == task_to_remove.title) {
            Some(self.list.remove(pos))
        } else {
            None // Task not found
        }
    }

    fn printTasks(&self) {
        for task in &self.list {
            task.printInfo();
        }
    }

    // Helper method to get overdue and upcoming tasks
    fn get_overdue_tasks(&mut self) -> Vec<&mut Task> {
        let now = Local::now();
        let current_date = NaiveDate::from_ymd_opt(
            now.year(),
            now.month(),
            now.day()
        ).unwrap();
        let current_time = NaiveTime::from_hms_opt(
            now.hour(),
            now.minute(),
            now.second()
        ).unwrap();
        let current_datetime = (current_date, current_time);

        self.list.iter_mut()
            .filter(|task| (task.date, task.time) < current_datetime)
            .collect()
    }

    fn get_upcoming_tasks(&mut self) -> Vec<&mut Task> {
        let now = Local::now();
        let current_date = NaiveDate::from_ymd_opt(
            now.year(),
            now.month(),
            now.day()
        ).unwrap();
        let current_time = NaiveTime::from_hms_opt(
            now.hour(),
            now.minute(),
            now.second()
        ).unwrap();
        let current_datetime = (current_date, current_time);

        self.list.iter_mut()
            .filter(|task| (task.date, task.time) >= current_datetime)
            .collect()
    }
    fn save_to_file(&self, path: &str) {
        let data = serde_json::to_string(&self.list).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
    fn load_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).unwrap_or("[]".to_string());
        if data.is_empty() {
            return TaskList::new(Vec::new());}
        else {
            let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();
            return TaskList::new(tasks);
        }
    }
}

pub struct MyApp {
    pub task_list: TaskList,
    username: String,
    color: egui::Color32,
    pub temp_task_title: String,
    pub temp_task_details: String,
    pub temp_task_time: NaiveTime,
    pub temp_task_date: NaiveDate,
    pub show_add_task_window: bool,
    pub show_completed_tasks: bool,
    pub temp_task_hour: u32,
    pub temp_task_minute: u32,

}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            task_list:TaskList::load_from_file("tasks.json"),
            username: "".to_string(),
            color: egui::Color32::LIGHT_BLUE,
            temp_task_title: "".to_string(),
            temp_task_details: "".to_string(),
            temp_task_time: NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            temp_task_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            show_add_task_window: false,
            show_completed_tasks: false,
            temp_task_hour: 0,
            temp_task_minute: 0,
        }
    }

}
impl MyApp {
    fn reset_temporary(&mut self) {
        self.temp_task_title = String::new();
        self.temp_task_details = String::new();
        self.temp_task_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        self.temp_task_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        self.show_add_task_window = false;
        self.temp_task_hour= 0;
        self.temp_task_minute= 0;


    }
}
fn TaskRectMake(ui: &mut egui::Ui, mut task: &mut Task) {
    // Create a frame with some padding
    let frame = egui::Frame::none()
        .fill(task.color)
        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
        .rounding(8.0);
        // .inner_margin(egui::style::Margin::same(10.0));

    frame.show(ui, |ui| {
        ui.vertical(|ui| {
            // Display task title in bold
            ui.heading(&task.title);

            // Display task details
            ui.label(&task.details);

            // Display date and time
            ui.horizontal(|ui| {
                ui.label(format!("Date: {}", task.date.format("%Y-%m-%d")));
                ui.label(format!("Time: {}", task.time.format("%H:%M")));
            });

            // Display completion status
            let status_text = if task.completed { "✓ Completed" } else { "⧖ Pending" };
            let status_color = if task.completed {
                egui::Color32::GREEN
            } else {
                egui::Color32::YELLOW
            };
            if task.completed {
                if ui.button("Uncomplete").clicked() {
                    task.completed = false;

                }

            }
            if  !task.completed{
                if ui.button("Complete").clicked() {
                    task.completed = true;
                }
            }
            ui.colored_label(status_color, status_text);

        });
    });

    // Add some space after each task
    ui.add_space(8.0);
}

// Now modify the eframe::App implementation to use these methods
impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sort tasks before displaying
        self.task_list.save_to_file("tasks.json");
        self.task_list.sort_by_status_and_magnitude();
        if self.show_add_task_window {
            egui::Window::new("Add Task").resizable(true).show(ctx, |ui| {
                ui.heading("Add New Task");
                ui.separator();
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.temp_task_title);
                ui.separator();
                ui.label("Details:");
                ui.text_edit_multiline(&mut self.temp_task_details);
                ui.separator();
                ui.label("Date:");
                ui.add(DatePickerButton::new(&mut self.temp_task_date));
                ui.separator();
                ui.heading("Time:");
                ui.horizontal(|ui| {
                    ui.label("hour");
                    ui.add(egui::Slider::new(&mut self.temp_task_hour,0..=23).suffix(" h"));
                    ui.separator();
                    ui.label("minute");
;                    ui.add(egui::Slider::new(&mut self.temp_task_minute,0..=59).suffix(" min"));
                });
                ui.vertical(|ui| {
                    if ui.button("Confirm").clicked() {
                        self.temp_task_time = NaiveTime::from_hms_opt(self.temp_task_hour, self.temp_task_minute, 0).unwrap();
                         let task=Task::new(&self.temp_task_title,&self.temp_task_details,self.color,self.temp_task_date,self.temp_task_time,false);

                        self.task_list.add_task(task);
                        self.task_list.save_to_file("tasks.json");
                        self.reset_temporary();
                    }
                    else if ui.button("Cancel").clicked() {
                        // Handle cancel action here
                        self.reset_temporary();
                    }
                });
            });
        }
        if self.show_completed_tasks {
            egui::Window::new("Completed Tasks").resizable(true).show(ctx, |ui| {
                for task in &mut self.task_list.list {
                    if task.completed {
                        TaskRectMake(ui, task);
                    }
                }
                if ui.button("Exit").clicked() {
                    self.show_completed_tasks = false;
                }
            });}
        egui::SidePanel::left("side_panel_left")
            .frame(egui::Frame::default().fill(self.color))
            .show(ctx, |ui| {
                ui.heading("");
                if ui.button("Add Task").clicked() {
                    self.show_add_task_window = true;
                }
                if ui.button("Show Completed Tasks").clicked() {
                    self.show_completed_tasks = true;
                }
            });

        egui::SidePanel::right("side_panel_right")
            .frame(egui::Frame::default().fill(self.color))
            .show(ctx, |ui| {
                ui.heading("");
            });

        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::default().fill(self.color))
            .show(ctx, |ui| {
                ui.heading("NOTIF APP");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");

            // Get upcoming and overdue tasks
            let now = Local::now();
            let current_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
            let current_time = NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap();
            let current_datetime = (current_date, current_time);

            let (mut overdue_tasks, mut upcoming_tasks): (Vec<_>, Vec<_>) =
                self.task_list.list.iter_mut().partition(|task| (task.date, task.time) < current_datetime);

            egui::SidePanel::left("SoonTasks").default_width(300.0).show_inside(ui, |ui| {
                ui.heading("Soon");

                if upcoming_tasks.is_empty() {
                    ui.label("No upcoming tasks");
                } else {
                    // Display upcoming tasks
                    for task in upcoming_tasks.iter_mut() {
                        if !task.completed {
                            TaskRectMake(ui,  task);}
                    }
                }
            });

            egui::SidePanel::right("TooLateTasks").default_width(300.0).show_inside(ui, |ui| {
                ui.heading("Late");

                if overdue_tasks.is_empty() {
                    ui.label("No overdue tasks");
                } else {
                    // Display overdue tasks
                    for task in overdue_tasks.iter_mut() {
                        if !task.completed {
                        TaskRectMake(ui, task);}
                    }
                }
            });
        });
    }
}
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };

    // Create and initialize your app
    let mut app = MyApp::default();

    eframe::run_native(
        "NOTIF APP",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // Use the app we already initialized
            Ok(Box::new(app))
        }),
    )}
