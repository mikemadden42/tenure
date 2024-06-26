use chrono::offset::{TimeZone, Utc};
use chrono::NaiveDateTime;

fn main() {
    let d1 = Utc::now();
    let naive_d2 =
        NaiveDateTime::parse_from_str("Jul 6 08:00:00 2020", "%b %d %H:%M:%S %Y").unwrap();
    let d2 = Utc.from_utc_datetime(&naive_d2);
    let duration = d2.signed_duration_since(d1);
    println!("diff: {:?} days", duration.num_days().abs());
}
