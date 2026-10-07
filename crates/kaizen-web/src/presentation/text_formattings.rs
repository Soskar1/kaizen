pub fn format_hours_minutes(duration: u64) -> String {
    let minutes = (duration % 3600) / 60;
    let hours = duration / 3600;

    match (hours, minutes) {
        (0, minutes) => format!("{minutes}m"),
        _ => format!("{hours}h {minutes}m")
    }
}

pub fn format_hours_minutes_seconds(duration: u64) -> String {
    let hours = duration / 3600;
    let minutes = (duration % 3600) / 60;
    let seconds = duration % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}