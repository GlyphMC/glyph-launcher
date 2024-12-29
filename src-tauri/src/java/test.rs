use std::{path::PathBuf, process::Command};

use anyhow::Error;
use regex::Regex;

pub fn test_java(paths: (PathBuf, PathBuf, PathBuf)) -> Result<(bool, bool, bool), Error> {
	let paths = [(paths.0, 8), (paths.1, 17), (paths.2, 21)];
	let mut results = (false, false, false);

	for (i, (path, _version)) in paths.iter().enumerate() {
		let valid = if !path.as_os_str().is_empty() {
			let output = Command::new(path)
				.arg("-version")
				.output()?;

			let output_str = String::from_utf8_lossy(&output.stderr);
			let re = Regex::new(r"(?i)java version|openjdk version").unwrap();
			re.is_match(&output_str)
		} else {
			false
		};

		match i {
			0 => results.0 = valid,
			1 => results.1 = valid,
			2 => results.2 = valid,
			_ => unreachable!(),
		}
	}

	Ok(results)
}
