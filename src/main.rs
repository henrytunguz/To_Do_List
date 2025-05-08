use chrono::{Datelike, Local, NaiveDate, NaiveTime, Timelike};
use eframe::{egui, epaint};
use egui_extras;
use egui::{containers::panel, Ui};
#[derive(Clone)]
struct Task {
    title: String,
    details: String,
    color: [f32; 3],
    date: NaiveDate,
    time: NaiveTime,
    completed: bool,
}

impl Task {
    fn new(
        title: &str,
        details: &str,
        color: [f32; 3],
        date: [u32; 3],
        time: [u32; 3],
        completed: bool,
    ) -> Self {
        Task {
            title: title.to_string(),
            details: details.to_string(),
            color: color,
            date: NaiveDate::from_ymd_opt(date[0] as i32, date[1], date[2]).unwrap(),
            time: NaiveTime::from_hms_opt(time[0], time[1], time[2]).unwrap(),

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
        self.sort_by_datetime()
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
}
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h % 360.0) as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}
pub struct MyApp {
    pub task_list: TaskList,
    username: String,
    color: egui::Color32,

}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            task_list: TaskList::new(Vec::new()),
            username: "".to_string(),
            color: egui::Color32::LIGHT_BLUE,
        }
    }
}
fn TaskRectMake(ui: &mut egui::Ui, task: &Task) {
    // Create a frame with some padding
    let frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(
            (task.color[0]) as u8,
            (task.color[1]) as u8,
            (task.color[2]) as u8,
        ))
        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
        .rounding(8.0);


    frame.show(ui, |ui| {        ui.vertical_centered(|ui| {
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

            ui.colored_label(status_color, status_text);
        });
    });

    // Add some space after each task
    ui.add_space(8.0);
}fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };

    // Create and initialize your app
    let mut app = MyApp::default();
    let task = Task::new(
        "homework",
        "do math pages 1-11",
        [150.0, 250.0, 200.0],
        [2024, 2, 3],
        [13, 30, 0],
        false,
    );
    app.task_list.add_task(task);

    eframe::run_native(
        "NOTIF APP",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // Use the app we already initialized
            Ok(Box::new(app))
        }),
    )}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // In your update method or wherever you prepare data for display
        self.task_list.sort_by_status_and_magnitude();

        egui::SidePanel::left("side_panel_left")
            .frame(egui::Frame::default().fill(self.color))
            .show(ctx, |ui|
             {
                ui.heading("");
            });
        egui::SidePanel::right("side_panel_right")
            .frame(egui::Frame::default().fill(self.color))
            .show(ctx, |ui|
                {
                    ui.heading("");
                });
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::default().fill(self.color))
            .show(ctx, |ui|
                {
                    ui.heading("NOTIF APP" );
                });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tasks");
            egui::SidePanel::left("SoonTasks").default_width(300.0).show_inside(ui, |ui| {
                ui.heading("Soon");
                // if let Some(task) = self.task_list.list.get(0) {
                //     ui.label(&task.title);
                // }
                for task in &self.task_list.list {
                    TaskRectMake(ui, task);
                }

            });
            egui::SidePanel::right("TooLateTasks").default_width(300.0).show_inside(ui, |ui| {
                ui.heading("Late");
                ui.label("LATE!");
            });
        });
    }
}