extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let billion = 1000000000;
    let billion_duration = Duration::seconds(billion);
    let date_time_plus_billion = start.checked_add_signed(billion_duration);
    match date_time_plus_billion.is_some() {
        true => return date_time_plus_billion.unwrap(),
        false => panic!("Unsupported number of seconds"),
    }
   
}
