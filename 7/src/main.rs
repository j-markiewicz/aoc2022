use std::{
	collections::HashMap,
	error::Error,
	fs,
	io::{Error as IoError, ErrorKind},
};

use regex::RegexBuilder;

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let re = RegexBuilder::new(r"^(\$ (cd) (.+))|(((\d+)|(dir)) (.+))$")
		.multi_line(true)
		.build()?;

	let mut sizes = HashMap::<String, u64>::new();
	let mut dir_sizes = HashMap::<String, u64>::new();
	let mut loc = Vec::<String>::new();

	for r in re.captures_iter(&input) {
		match (r.get(2).map(|r| r.as_str()), r.get(3).map(|r| r.as_str())) {
			(Some("cd"), Some("..")) => {
				loc.pop();
			}
			(Some("cd"), Some("/")) => {
				loc.clear();
			}
			(Some("cd"), Some(dir)) => {
				loc.push(dir.to_string());
			}
			_ => {
				if let (Some(size), Some(name)) =
					(r.get(6).map(|r| r.as_str()), r.get(8).map(|r| r.as_str()))
				{
					let mut path = String::new();
					for l in &loc {
						path += "/";
						path += l;
					}
					sizes.insert(path + "/" + name, size.parse()?);
				}
			}
		}
	}

	for (name, size) in &sizes {
		let path = name.split('/').map(ToString::to_string).collect::<Vec<_>>();

		for n in 1..path.len() {
			let dir_name = path
				.get(0..n)
				.ok_or_else(|| IoError::from(ErrorKind::NotFound))?
				.join("/");

			if dir_sizes.contains_key(&dir_name) {
				*dir_sizes
					.get_mut(&dir_name)
					.ok_or_else(|| IoError::from(ErrorKind::NotFound))? += size;
			} else {
				dir_sizes.insert(dir_name.to_string(), *size);
			}
		}
	}

	let total_size: u64 = dir_sizes.values().copied().filter(|&s| s <= 100_000).sum();
	println!("1: {total_size}");

	let space_needed = 30_000_000
		- (70_000_000
			- dir_sizes
				.get("")
				.ok_or_else(|| IoError::from(ErrorKind::NotFound))?);

	let mut dirs = dir_sizes
		.values()
		.copied()
		.filter(|&s| s >= space_needed)
		.collect::<Vec<_>>();

	dirs.sort_unstable();
	println!("2: {}", dirs[0]);

	Ok(())
}
