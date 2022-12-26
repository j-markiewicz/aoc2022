use std::{
	collections::HashSet,
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs,
};

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("empty data")
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
	Up,
	Down,
	Left,
	Right,
}

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);
const U_D_L_R: [(isize, isize); 4] = [UP, DOWN, LEFT, RIGHT];

macro_rules! add {
	($a:expr, $b:expr) => {{
		let a: (usize, usize) = $a;
		let b: (isize, isize) = $b;
		if let Some(x) = a.0.checked_add_signed(b.0) {
			if let Some(y) = a.1.checked_add_signed(b.1) {
				Some((x, y))
			} else {
				None
			}
		} else {
			None
		}
	}};
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?.trim().to_string();
	let mut walls = Vec::new();
	let mut blizzards = Vec::new();
	let width = input.lines().last().ok_or(EmptyError)?.len() - 1;
	let height = input.lines().count() - 1;
	let goal = (width - 1, height);

	for (i, line) in input.lines().enumerate() {
		for (j, char) in line.chars().enumerate() {
			match char {
				'#' => {
					walls.push((j, i));
				}
				'^' => {
					blizzards.push(((j, i), Dir::Up));
				}
				'v' => {
					blizzards.push(((j, i), Dir::Down));
				}
				'<' => {
					blizzards.push(((j, i), Dir::Left));
				}
				'>' => {
					blizzards.push(((j, i), Dir::Right));
				}
				'.' => {}
				_ => panic!("invalid input"),
			}
		}
	}

	let mut positions = HashSet::new();
	positions.insert((1, 0));

	let mut i = 0;
	loop {
		i += 1;

		blizzards = blizzards
			.into_iter()
			.map(|(pos, dir)| {
				let mut new_pos = match dir {
					Dir::Up => add!(pos, UP).unwrap(),
					Dir::Down => add!(pos, DOWN).unwrap(),
					Dir::Left => add!(pos, LEFT).unwrap(),
					Dir::Right => add!(pos, RIGHT).unwrap(),
				};

				if walls.contains(&new_pos) {
					match dir {
						Dir::Up => new_pos = (new_pos.0, height - 1),
						Dir::Down => new_pos = (new_pos.0, 1),
						Dir::Left => new_pos = (width - 1, new_pos.1),
						Dir::Right => new_pos = (1, new_pos.1),
					}
				}

				(new_pos, dir)
			})
			.collect();

		positions = positions
			.iter()
			.flat_map(|&m| {
				let mut next = vec![m];
				for d in U_D_L_R {
					if let Some(np) = add!(m, d) {
						if !blizzards.iter().any(|&(p, _)| p == np) && !walls.contains(&np) {
							next.push(np);
						}
					}
				}
				next
			})
			.collect();

		if positions.contains(&goal) {
			break;
		}

		dbg!(positions.len());
	}

	println!("1: {i}");

	Ok(())
}
