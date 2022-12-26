use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	ops::Neg,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Facing {
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3,
}

impl Neg for Facing {
	type Output = Facing;

	fn neg(self) -> Self::Output {
		match self {
			Self::Right => Self::Left,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
			Self::Up => Self::Down,
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
	Empty,
	Wall,
	Path,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
	Walk(usize),
	SpinLeft,
	SpinRight,
}

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("empty data")
	}
}

fn print_map(map: &Vec<Vec<Tile>>) {
	for line in map {
		for tile in line {
			print!(
				"{}",
				match tile {
					Tile::Empty => ' ',
					Tile::Wall => '#',
					Tile::Path => '.',
				}
			);
		}

		println!();
	}
}

pub fn solve(input: &str, print: bool) -> Result<(), Box<dyn Error>> {
	let ml = input
		.lines()
		.take_while(|l| !l.trim().is_empty())
		.map(|l| l.len())
		.max()
		.ok_or(EmptyError)?;
	let mut map = vec![
		vec![Tile::Empty; ml + 2];
		input.lines().take_while(|l| !l.trim().is_empty()).count() + 2
	];

	for (i, line) in input
		.lines()
		.take_while(|l| !l.trim().is_empty())
		.enumerate()
	{
		for (j, char) in line.chars().enumerate() {
			map[i + 1][j + 1] = match char {
				' ' => Tile::Empty,
				'#' => Tile::Wall,
				'.' => Tile::Path,
				_ => Err(EmptyError)?,
			}
		}
	}

	let mut instructions = input
		.lines()
		.rev()
		.find(|l| !l.trim().is_empty())
		.ok_or(EmptyError)?
		.to_string();

	let mut v_inst = Vec::new();
	while !instructions.is_empty() {
		let is_dist = instructions.chars().next().ok_or(EmptyError)?.is_numeric();
		let inst = instructions
			.chars()
			.take_while(|c| c.is_numeric() == is_dist)
			.collect::<String>();

		instructions.drain(0..inst.len()).count();
		v_inst.push(match inst.as_str() {
			"L" => Instruction::SpinLeft,
			"R" => Instruction::SpinRight,
			d => Instruction::Walk(d.parse()?),
		});
	}

	let instructions = v_inst;

	if print {
		print_map(&map);
	}

	let mut facing = Facing::Right;
	let mut pos = (0usize, 0usize);

	'find_pos: for (i, line) in map.iter().enumerate() {
		for (j, tile) in line.iter().enumerate() {
			if *tile == Tile::Path {
				pos = (j, i);
				break 'find_pos;
			}
		}
	}

	for (_i, inst) in instructions.iter().copied().enumerate() {
		match inst {
			Instruction::SpinLeft => match facing {
				Facing::Right => facing = Facing::Up,
				Facing::Down => facing = Facing::Right,
				Facing::Left => facing = Facing::Down,
				Facing::Up => facing = Facing::Left,
			},
			Instruction::SpinRight => match facing {
				Facing::Right => facing = Facing::Down,
				Facing::Down => facing = Facing::Left,
				Facing::Left => facing = Facing::Up,
				Facing::Up => facing = Facing::Right,
			},
			Instruction::Walk(d) => {
				for _d in 0..d {
					match facing {
						Facing::Right => pos.0 += 1,
						Facing::Down => pos.1 += 1,
						Facing::Left => pos.0 -= 1,
						Facing::Up => pos.1 -= 1,
					}

					match map[pos.1][pos.0] {
						Tile::Wall => match facing {
							Facing::Right => pos.0 -= 1,
							Facing::Down => pos.1 -= 1,
							Facing::Left => pos.0 += 1,
							Facing::Up => pos.1 += 1,
						},
						Tile::Path => continue,
						Tile::Empty => {
							let Some((nf, np)) = edge(pos, facing) else {
								panic!(
									"fell off cube going {:?} from ({}, {}) during instruction {:?} / {}, after {:?}",
									facing,
									pos.0,
									pos.1,
									inst,
									_d,
									_i.checked_sub(1).map(|i| instructions[i])
								);
							};

							if map[np.1][np.0] == Tile::Empty {
								pos = np;
								facing = nf
							} else {
								match facing {
									Facing::Right => pos.0 -= 1,
									Facing::Down => pos.1 -= 1,
									Facing::Left => pos.0 += 1,
									Facing::Up => pos.1 += 1,
								}
							}
						}
					}
				}
			}
		}
	}

	println!("2: {}", 1000 * pos.1 + 4 * pos.0 + (facing as usize));

	Ok(())
}

/// Traverse the edge of the map as a cube
///
/// This is hard-coded and will only work if the input is the same shape as
/// the one provided in `input.txt`. This will not work for the example, and
/// it probably will not work for most other inputs.
fn edge(pos: (usize, usize), facing: Facing) -> Option<(Facing, (usize, usize))> {
	if pos.0 == 50 && (1..=50).contains(&pos.1) && facing == Facing::Left {
		Some((Facing::Right, (1, (50 - (pos.1 - 1)) + 100)))
	} else if pos.0 == 50 && (51..=100).contains(&pos.1) && facing == Facing::Left {
		Some((Facing::Down, (pos.1 - 50, 101)))
	} else if (1..=50).contains(&pos.0) && pos.1 == 100 && facing == Facing::Up {
		Some((Facing::Right, (51, pos.0 + 50)))
	} else if pos.0 == 0 && (101..=150).contains(&pos.1) && facing == Facing::Left {
		Some((Facing::Right, (51, (50 - (pos.1 - 101)))))
	} else if pos.0 == 0 && (151..=200).contains(&pos.1) && facing == Facing::Left {
		Some((Facing::Down, (pos.1 - 100, 1)))
	} else if (1..=50).contains(&pos.0) && pos.1 == 201 && facing == Facing::Down {
		Some((Facing::Down, (pos.0 + 100, 1)))
	} else if pos.0 == 51 && (151..=200).contains(&pos.1) && facing == Facing::Right {
		Some((Facing::Up, (pos.1 - 100, 150)))
	} else if (51..=100).contains(&pos.0) && pos.1 == 151 && facing == Facing::Down {
		Some((Facing::Left, (50, pos.0 + 100)))
	} else if pos.0 == 101 && (101..=150).contains(&pos.1) && facing == Facing::Right {
		Some((Facing::Left, (150, (50 - (pos.1 - 101)))))
	} else if pos.0 == 101 && (51..=100).contains(&pos.1) && facing == Facing::Right {
		Some((Facing::Up, (pos.1 + 50, 50)))
	} else if (101..=150).contains(&pos.0) && pos.1 == 51 && facing == Facing::Down {
		Some((Facing::Left, (100, pos.0 - 50)))
	} else if pos.0 == 151 && (1..=50).contains(&pos.1) && facing == Facing::Right {
		Some((Facing::Left, (100, (50 - (pos.1 - 1)) + 100)))
	} else if (101..=150).contains(&pos.0) && pos.1 == 0 && facing == Facing::Up {
		Some((Facing::Up, (pos.0 - 100, 200)))
	} else if (51..=100).contains(&pos.0) && pos.1 == 0 && facing == Facing::Up {
		Some((Facing::Right, (1, pos.0 + 100)))
	} else {
		None
	}
}
