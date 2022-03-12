extern crate chrono;
use chrono::offset::{TimeZone, Utc};

fn main() {
    let utc = Utc;
    let d1 = Utc::now();
    let d2 = utc
        .datetime_from_str("Jul 6 08:00:00 2020", "%b %d %H:%M:%S %Y")
        .unwrap();
    let duration = d2.signed_duration_since(d1);
    println!("diff: {:?} days", duration.num_days().abs());
}
