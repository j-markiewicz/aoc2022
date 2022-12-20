mod rocks;

use rocks::{Rock, ROCKS, ROCK_HEIGHT};
use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs,
};

const CHAMBER_WIDTH: usize = 7;
const CHAMBER_HEIGHT: usize = 512;
const STOP_1: usize = 2022;
#[cfg(feature = "i_have_a_lot_of_time")]
const STOP_2: usize = 1000000000000;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
	Left,
	Right,
}

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("empty data")
	}
}

fn simulate(input: impl AsRef<str>, stop: usize) -> Result<usize, Box<dyn Error>> {
	let mut directions = input
		.as_ref()
		.trim()
		.chars()
		.filter_map(|c| {
			if c == '>' {
				Some(Direction::Right)
			} else if c == '<' {
				Some(Direction::Left)
			} else {
				None
			}
		})
		.cycle();

	let mut chamber = vec![[Rock::None; CHAMBER_WIDTH]; CHAMBER_HEIGHT];
	let mut chamber_bottom = 0;

	for i in 0..stop {
		let mut highest = -1isize;
		for (j, row) in chamber.iter().enumerate().rev() {
			if *row != [Rock::None; 7] {
				highest = j.try_into()?;
				break;
			}
		}

		if chamber.len() - usize::try_from(highest).unwrap_or(0) < 10 {
			let new_bottom = chamber.len() / 2;
			chamber.copy_within(new_bottom.., 0);

			for row in chamber.iter_mut().skip(new_bottom) {
				*row = [Rock::None; CHAMBER_WIDTH];
			}

			chamber_bottom += new_bottom;
			highest = highest.checked_sub_unsigned(new_bottom).ok_or(EmptyError)?;
		}

		for j in 0..ROCK_HEIGHT {
			chamber[usize::try_from(highest + 4)? + j] =
				ROCKS[i % ROCKS.len()][ROCK_HEIGHT - j - 1];
		}

		'outer: loop {
			let dir = directions.next().ok_or(EmptyError)?;
			let mut can_move = true;

			for row in chamber.iter_mut() {
				for j in 0..row.len() {
					if row[j] == Rock::Falling {
						match dir {
							Direction::Left => {
								if j == 0 || row[j - 1] == Rock::Stopped {
									can_move = false;
								}
							}
							Direction::Right => {
								if j == row.len() - 1 || row[j + 1] == Rock::Stopped {
									can_move = false;
								}
							}
						}
					}
				}
			}

			if can_move {
				for row in chamber.iter_mut() {
					match dir {
						Direction::Left => {
							for j in 1..row.len() {
								if row[j] == Rock::Falling {
									row[j - 1] = Rock::Falling;
									row[j] = Rock::None;
								}
							}
						}
						Direction::Right => {
							for j in (0..row.len() - 1).rev() {
								if row[j] == Rock::Falling {
									row[j + 1] = Rock::Falling;
									row[j] = Rock::None;
								}
							}
						}
					}
				}
			}

			let mut can_fall = true;

			for j in 0..chamber.len() {
				for k in 0..chamber[j].len() {
					if chamber[j][k] == Rock::Falling
						&& (j == 0 || chamber[j - 1][k] == Rock::Stopped)
					{
						can_fall = false;
					}
				}
			}

			if can_fall {
				for j in 0..chamber.len() {
					for k in 0..chamber[j].len() {
						if chamber[j][k] == Rock::Falling {
							chamber[j - 1][k] = Rock::Falling;
							chamber[j][k] = Rock::None;
						}
					}
				}
			} else {
				break 'outer;
			}
		}

		for row in chamber.iter_mut() {
			for rock in row {
				if *rock == Rock::Falling {
					*rock = Rock::Stopped
				}
			}
		}
	}

	let mut highest = 0;
	for (j, row) in chamber.iter().enumerate() {
		if *row == [Rock::None; 7] {
			highest = j;
			break;
		}
	}

	Ok(highest + chamber_bottom)
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	#[cfg(feature = "i_have_a_lot_of_time")]
	for rock in ROCKS {
		for row in rock {
			for stone in row {
				print!("{stone}");
			}
			println!();
		}
		println!();
	}

	let highest_1 = simulate(&input, STOP_1)?;
	println!("1: {highest_1}");

	#[cfg(feature = "i_have_a_lot_of_time")]
	{
		let highest_2 = simulate(&input, STOP_2)?;
		println!("2: {highest_2}");
	}
	#[cfg(not(feature = "i_have_a_lot_of_time"))]
	{
		println!("2: ???");
		println!("2: If you have approximately 3000-5000 hours, use");
		println!("2$ cargo run --release --features i_have_a_lot_of_time");
		println!("2: to calculate the solution to part 2.");
	}

	Ok(())
}
