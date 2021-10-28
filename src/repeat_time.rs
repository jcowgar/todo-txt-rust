use std::ops::Add;

use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use regex::Regex;

lazy_static! {
    static ref REPEAT_PATTERN_RE: Regex = Regex::new(r"^(?P<frequency>\d+)(?P<unit>[dwmy])$").unwrap();
}

/// Step through
fn compute_month(ref_d: NaiveDate, months: u32) -> NaiveDate {
	let mut month = ref_d.month();
	let mut year = ref_d.year();

	for _ in 0..months {
		month += 1;
		if month > 12 {
			year += 1;
			month = 1;
		}
	}

	NaiveDate::from_ymd(year, month, ref_d.day())
}

/// Compute the next date based on reference date (or now)
/// and the repeat pattern.
///
/// Patterns supported:
///
///   1d = every day
///   7d = every 7 days
///   1w = every week
///   1m = every month
///   1y = every year
pub fn next_date(pattern: &str, reference_date: Option<NaiveDate>) -> Option<NaiveDate> {
	let ref_d: NaiveDate = match reference_date {
		None => Local::today().naive_local(),
		Some(v) => v,
	};

	match REPEAT_PATTERN_RE.captures(pattern) {
		None => None,
		Some(matches) => {
			let frequency = matches.name("frequency")?.as_str().parse::<u32>().unwrap();
			let unit = matches.name("unit")?.as_str();

			match unit {
				"d" => Some(ref_d.add(Duration::days(frequency as i64))),
				"w" => Some(ref_d.add(Duration::weeks(frequency as i64))),
				"m" => Some(compute_month(ref_d, frequency)),
				"y" => Some(NaiveDate::from_ymd(ref_d.year() + frequency as i32, ref_d.month(), ref_d.day())),
				_ => None,
			}
		}
	}
}

pub fn next_weekday(weekday: Weekday, reference_date: Option<NaiveDate>) -> NaiveDate {
	let ref_d: NaiveDate = match reference_date {
		None => Local::today().naive_local(),
		Some(v) => v,
	};
	let current_weekday = ref_d.weekday();
	let mut diff = weekday.num_days_from_monday() as i32 - current_weekday.num_days_from_monday() as i32;

	if diff < 0 {
		diff += 7;
	}

	ref_d.add(Duration::days(diff as i64))
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_1_day() {
		let repeat_pattern = "1d";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 2);
    }

    #[test]
    fn test_14_days() {
		let repeat_pattern = "14d";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 15);
    }

    #[test]
    fn test_1_week() {
		let repeat_pattern = "1w";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 8);
    }

    #[test]
    fn test_10_weeks() {
		let repeat_pattern = "10w";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2021, "year");
		assert_eq!(n.month(), 3, "month");
		assert_eq!(n.day(), 12, "day");
    }

    #[test]
    fn test_1_month() {
		let repeat_pattern = "1m";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 2);
		assert_eq!(n.day(), 1);
    }

    #[test]
    fn test_2_months() {
		let repeat_pattern = "2m";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2021, "year");
		assert_eq!(n.month(), 3, "month");
		assert_eq!(n.day(), 1, "day");
    }

    #[test]
    fn test_13_months() {
		let repeat_pattern = "13m";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2022, "year");
		assert_eq!(n.month(), 2, "month");
		assert_eq!(n.day(), 1, "day");
    }

    #[test]
    fn test_1_year() {
		let repeat_pattern = "1y";
		let n = next_date(repeat_pattern, Some(NaiveDate::from_ymd(2021, 1, 1))).unwrap();

		assert_eq!(n.year(), 2022);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 1);
    }

	#[test]
	fn test_next_weekday_fri_to_sat() {
		let n = next_weekday(Weekday::Sat, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 2);
	}

	#[test]
	fn test_next_weekday_fri_to_sun() {
		let n = next_weekday(Weekday::Sun, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 3);
	}

	#[test]
	fn test_next_weekday_fri_to_mon() {
		let n = next_weekday(Weekday::Mon, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 4);
	}

	#[test]
	fn test_next_weekday_fri_to_tue() {
		let n = next_weekday(Weekday::Tue, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 5);
	}

	#[test]
	fn test_next_weekday_fri_to_wed() {
		let n = next_weekday(Weekday::Wed, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 6);
	}

	#[test]
	fn test_next_weekday_fri_to_thu() {
		let n = next_weekday(Weekday::Thu, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 7);
	}

	#[test]
	fn test_next_weekday_fri_to_fri() {
		let n = next_weekday(Weekday::Fri, Some(NaiveDate::from_ymd(2021, 1, 1)));

		assert_eq!(n.year(), 2021);
		assert_eq!(n.month(), 1);
		assert_eq!(n.day(), 1);
	}
}
