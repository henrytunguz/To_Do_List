use chrono::{Datelike, Local, NaiveDate, NaiveTime, Timelike};

struct Task {
    title: String,
    details: String,
    color: [f32; 3], // HSL values
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

    fn printInfo(tsk: Task) {
        println!("Title: {}", tsk.title);
        println!("Details: {}", tsk.details);
        println!("Color: {:?}", tsk.color);
        println!("Date: {}", tsk.date);
        println!("Time: {}", tsk.time);
        println!("Completed: {}", tsk.completed);
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

fn main() {
    println!("Hello, world!");
    let now = Local::now();
    let tsk = Task::new(
        "homework",
        "do math pages 1-11",
        [150.0, 250.0, 200.0],
        [2024, 2, 3],
        [13, 30, 0],
        false,
    );
    let task1: Task;
    let date = now.date_naive();
    let dateList: [u32; 3];
    dateList = [now.year() as u32, now.month(), now.day()];
    let timeList: [u32; 3];
    timeList = [now.hour(), now.minute(), now.second()];
    task1 = Task::new(
        "today",
        "how are you",
        [10.0, 101.0, 10.0],
        dateList,
        timeList,
        false,
    );

    Task::printInfo(tsk);
    Task::printInfo(task1);
}
