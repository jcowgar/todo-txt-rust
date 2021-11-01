use regex::Regex;

lazy_static! {
	static ref HMS_RS: Regex =
		Regex::new(r"^((?P<hours>\d+)h)?((?P<minutes>\d+)m)?((?P<seconds>\d+)s)?$").unwrap();
}

/// Convert an XhXmXs string to seconds
pub fn to_seconds(hms: &String) -> i64 {
	let mut total_seconds: i64 = 0;
	let matches = HMS_RS.captures(&hms);

	if let Some(m) = matches {
		match m.name("hours") {
			Some(v) => total_seconds += v.as_str().parse::<i64>().unwrap() * 3600,
			None => (),
		}
		match m.name("minutes") {
			Some(v) => total_seconds += v.as_str().parse::<i64>().unwrap() * 60,
			None => (),
		}
		match m.name("seconds") {
			Some(v) => total_seconds += v.as_str().parse::<i64>().unwrap(),
			None => (),
		}
	}

	return total_seconds;
}

/// Convert seconds to an XhXmXs string
pub fn from_seconds(seconds: i64) -> String {
	let hours = seconds / 3600;
	let minutes = (seconds - (hours * 3600)) / 60;
	let seconds = seconds - (hours * 3600) - (minutes * 60);

	let parts: &mut Vec<String> = &mut vec![];

	if hours > 0 {
		parts.push(format!("{}h", hours));
	}

	if minutes > 0 {
		parts.push(format!("{}m", minutes));
	}

	if seconds > 0 {
		parts.push(format!("{}s", seconds));
	}

	return parts.join("");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hms_to_seconds() {
		let seconds = to_seconds(&"1h1m1s".to_string());
		assert_eq!(seconds, 3661);
	}

	#[test]
	fn ms_to_seconds() {
		let seconds = to_seconds(&"1m1s".to_string());
		assert_eq!(seconds, 61);
	}

	#[test]
	fn s_to_seconds() {
		let seconds = to_seconds(&"1s".to_string());
		assert_eq!(seconds, 1);
	}

	#[test]
	fn from_seconds_hms() {
		let result = from_seconds(3661);

		assert_eq!(result, "1h1m1s");
	}

	#[test]
	fn from_seconds_ms() {
		let result = from_seconds(61);

		assert_eq!(result, "1m1s");
	}

	#[test]
	fn from_seconds_s() {
		let result = from_seconds(1);

		assert_eq!(result, "1s");
	}
}
