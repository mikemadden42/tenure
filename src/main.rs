use chrono::{Local, NaiveDate};
use std::{env, process};

const DEFAULT_START: &str = "2020-07-06";
const DATE_FORMAT: &str = "%Y-%m-%d";

fn days_between(start: NaiveDate, end: NaiveDate) -> i64 {
    (end - start).num_days()
}

fn main() {
    let arg = env::args().nth(1);
    if matches!(arg.as_deref(), Some("-h" | "--help")) {
        println!("Usage: tenure [START_DATE]\n\nPrint the number of days from START_DATE (YYYY-MM-DD, default {DEFAULT_START}) to today.");
        return;
    }
    let input = arg.as_deref().unwrap_or(DEFAULT_START);
    let start = NaiveDate::parse_from_str(input, DATE_FORMAT).unwrap_or_else(|e| {
        eprintln!("invalid date {input:?} (expected YYYY-MM-DD): {e}");
        process::exit(1);
    });
    let today = Local::now().date_naive();
    println!("{} days", days_between(start, today));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn same_day_is_zero() {
        assert_eq!(days_between(date(2020, 7, 6), date(2020, 7, 6)), 0);
    }

    #[test]
    fn counts_across_leap_year() {
        assert_eq!(days_between(date(2020, 1, 1), date(2021, 1, 1)), 366);
    }

    #[test]
    fn future_start_is_negative() {
        assert_eq!(days_between(date(2020, 7, 7), date(2020, 7, 6)), -1);
    }
}
