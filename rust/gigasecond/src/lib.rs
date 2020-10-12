extern crate chrono;
use chrono::*;

pub fn after(start_date: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond = Duration::seconds(i64::pow(10, 9));
    start_date.checked_add_signed(gigasecond).unwrap()
}
