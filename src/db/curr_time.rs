use std::time::{Duration, SystemTime, UNIX_EPOCH};

// yes yes we are downcasting to an i64, if this is somehow still used
// in 500 years then peeps can just use seconds instead of milis
// or just upgrade to i128 or whatever they use in 500 years
pub fn get_current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis() as i64
}

pub fn get_expiry(days: u64) -> i64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");
    let duration = Duration::from_secs(days * 60 * 60 * 12);
    let val = now + duration;
    val.as_millis() as i64
}
