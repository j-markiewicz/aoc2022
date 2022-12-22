use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs,
	str::FromStr,
};

const KEY: i64 = 811_589_153;

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("empty data")
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?
		.lines()
		.enumerate()
		.map(|(i, l)| Ok((i, l.parse::<i64>()?)))
		.collect::<Result<Vec<_>, <i64 as FromStr>::Err>>()?;
	let mut coords = input.clone();
	let l = coords.len();

	for i in 0..l {
		let j = coords.iter().position(|&(j, _)| i == j).ok_or(EmptyError)?;
		let m = coords.remove(j);

		coords.insert(
			usize::try_from(
				(isize::try_from(j)? + isize::try_from(m.1)?).rem_euclid(isize::try_from(l - 1)?),
			)?,
			m,
		);
	}

	let i = coords.iter().position(|&(_, e)| e == 0).ok_or(EmptyError)?;

	println!(
		"1: {} + {} + {} = {}",
		coords[(i + 1000) % l].1,
		coords[(i + 2000) % l].1,
		coords[(i + 3000) % l].1,
		coords[(i + 1000) % l].1 + coords[(i + 2000) % l].1 + coords[(i + 3000) % l].1
	);

	let mut coords = input
		.into_iter()
		.map(|(i, n)| (i, n * KEY))
		.collect::<Vec<_>>();

	for _ in 0..10 {
		for i in 0..l {
			let j = coords.iter().position(|&(j, _)| i == j).ok_or(EmptyError)?;
			let m = coords.remove(j);

			coords.insert(
				usize::try_from(
					(isize::try_from(j)? + isize::try_from(m.1)?)
						.rem_euclid(isize::try_from(l - 1)?),
				)?,
				m,
			);
		}
	}

	let i = coords.iter().position(|&(_, e)| e == 0).ok_or(EmptyError)?;

	println!(
		"2: {} + {} + {} = {}",
		coords[(i + 1000) % l].1,
		coords[(i + 2000) % l].1,
		coords[(i + 3000) % l].1,
		coords[(i + 1000) % l].1 + coords[(i + 2000) % l].1 + coords[(i + 3000) % l].1
	);

	Ok(())
}
