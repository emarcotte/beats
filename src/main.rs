use chrono::{Local, Timelike};

fn main() {
    let now = Local::now();

    let hour = std::env::args()
        .skip(1)
        .next()
        .map(|h_str| h_str.parse::<f32>().expect("Could not convert hours to number"))
        .unwrap_or_else(|| now.hour() as f32);
    let minute = std::env::args()
        .skip(2)
        .next()
        .map(|m_str| m_str.parse::<f32>().expect("Could not convert minutes to number"))
        .unwrap_or_else(|| now.minute() as f32);

    let offset_seconds = now.offset().utc_minus_local() as f32;
    let seconds = (hour * 3600.0) + (minute * 60.0)
        + 3600.0
        + offset_seconds;
    let beats = seconds / 86.4;
    println!("@{:04.1}", beats);
}
