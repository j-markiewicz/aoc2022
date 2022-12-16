use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs,
	ops::RangeInclusive,
	sync::atomic::{AtomicU64, Ordering},
};

use ahash::{HashSet, HashSetExt};
use rayon::prelude::*;
use regex::Regex;

/// If the first result is wrong, increase this until it isn't
const LARGE_NUMBER: i64 = 100000000;

const LINE_1: i64 = 2000000;
const AREA_2: RangeInclusive<i64> = 0..=4000000;
const FACTOR_2: i64 = 4000000;

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("empty data")
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let re = Regex::new(
		"Sensor at x=(?P<sx>-?\\d+), y=(?P<sy>-?\\d+): \
		closest beacon is at x=(?P<bx>-?\\d+), y=(?P<by>-?\\d+)",
	)?;

	let mut items = Vec::<(i64, i64, i64, i64)>::new();

	for line in input.lines() {
		let captures = re.captures(line).ok_or(EmptyError)?;

		items.push((
			captures.name("sx").ok_or(EmptyError)?.as_str().parse()?,
			captures.name("sy").ok_or(EmptyError)?.as_str().parse()?,
			captures.name("bx").ok_or(EmptyError)?.as_str().parse()?,
			captures.name("by").ok_or(EmptyError)?.as_str().parse()?,
		));
	}

	let (min_x, max_x, _min_y, _max_y) = items
		.iter()
		.map(|&(sx, sy, bx, by)| (sx.min(bx), sx.max(bx), sy.min(by), sy.max(by)))
		.reduce(
			|(min_x_a, max_x_a, min_y_a, max_y_a), (min_x_b, max_x_b, min_y_b, max_y_b)| {
				(
					min_x_a.min(min_x_b),
					max_x_a.max(max_x_b),
					min_y_a.min(min_y_b),
					max_y_a.max(max_y_b),
				)
			},
		)
		.ok_or(EmptyError)?;

	let mut no_beacons = HashSet::<(i64, i64)>::new();
	for &(sx, sy, bx, by) in &items {
		let beacon_distance = distance(sx, sy, bx, by);

		for x in (min_x - LARGE_NUMBER)..(max_x + LARGE_NUMBER) {
			if distance(x, LINE_1, sx, sy) <= beacon_distance {
				no_beacons.insert((x, LINE_1));
			}
		}
	}

	for &(sx, sy, bx, by) in &items {
		no_beacons.remove(&(sx, sy));
		no_beacons.remove(&(bx, by));
	}

	println!("1: {}", no_beacons.len());

	// This is probably the worst way to solve this that actually works, but at
	// least it's a decent way to heat your home. I swear I sometimes write good
	// code. Sample output (before some formatting changes; took ~5.5h to run):
	// https://imgur.com/a/HM39caH
	let n = AtomicU64::new(0);
	let beacon = AREA_2
		.into_par_iter()
		.find_map_any(|x| {
			let i = n.fetch_add(1, Ordering::Relaxed);
			if i % 4000 == 0 {
				println!(
					"2: ... searching - {:03.1}%",
					(i64::try_from(i).ok()? * 1000 / AREA_2.last()?) as f64 / 10.0
				);
			}

			for y in AREA_2 {
				let mut beacon_here = true;

				for &(sx, sy, bx, by) in &items {
					if distance(x, y, sx, sy) <= distance(sx, sy, bx, by) {
						beacon_here = false;
						break;
					}
				}

				if beacon_here {
					return Some((x, y));
				}
			}

			None
		})
		.ok_or(EmptyError)?;

	println!("2: {}", beacon.0 * FACTOR_2 + beacon.1);

	Ok(())
}

fn distance(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
	(ax.abs_diff(bx) + ay.abs_diff(by))
		.try_into()
		.expect("distance too large")
}
