use chrono::NaiveDate;
use chrono::Datelike;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let leap = NaiveDate::from_ymd_opt(year as i32, 2, 29).is_some();
    if leap {
        return None;
    }
    Some(NaiveDate::from_yo_opt(year as i32, 183).unwrap().weekday())
}