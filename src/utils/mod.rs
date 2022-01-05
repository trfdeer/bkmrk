pub mod files;

use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;

pub fn squeeze_whitespaces(text: &str) -> String {
    let ws = Regex::new(r"[\s\n]+").unwrap();
    let new_str = ws.replace_all(text, " ");
    new_str.into_owned()
}

pub fn get_date_string(ts: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(ts, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
