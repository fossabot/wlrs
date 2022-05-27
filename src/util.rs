/* unused
use chrono::{DateTime, NaiveDateTime, Utc};
pub fn timestamp_to_iso8601(timestamp: i64) -> String {
    let datetime: DateTime<Utc> = chrono::DateTime::from_utc(
        NaiveDateTime::from_timestamp(timestamp, 0),
        Utc,
    );
    datetime.format("%Y-%m-%d %H:%M").to_string()
}


pub fn human_duration(start: u64, end: u64) -> String {
    let duration = end - start;
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 3600) % 24;
    format!("{}h {}m {}s", hours, minutes, seconds)
}
*/

pub fn string_capital_case(word: &str) -> String {
    let mut capitalized: String = String::new();
    for (i, c) in word.chars().enumerate() {
        if i == 0 {
            capitalized.push(c.to_ascii_uppercase());
        } else {
            capitalized.push(c);
        }
    }
    capitalized
}
