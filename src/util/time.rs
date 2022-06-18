use chrono::Duration;

pub fn seconds_to_time_string(seconds: i64) -> String {
    let duration = Duration::seconds(seconds);

    let hours = duration.num_hours();
    let minutes = duration.num_minutes() - (hours * 60);
    let seconds = duration.num_seconds() - (minutes * 60);

    let mut time_string = format!("{:0>2}:{:0>2}", minutes, seconds);

    if hours != 0 {
        time_string = format!("{:0>2}:{}", hours, time_string)
    }

    time_string
}
