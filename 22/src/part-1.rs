use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Facing {
	Right = 0,
	Down = 1,
	Left = 2,
	Up = 3,
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

	for inst in instructions {
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
				for _ in 0..d {
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
						Tile::Empty => match facing {
							Facing::Right => {
								for (i, tile) in map[pos.1].iter().enumerate() {
									if *tile == Tile::Path {
										pos = (i, pos.1);
										break;
									} else if *tile == Tile::Wall {
										pos.0 -= 1;
										break;
									}
								}
							}
							Facing::Down => {
								for (j, line) in map.iter().enumerate() {
									if line[pos.0] == Tile::Path {
										pos = (pos.0, j);
										break;
									} else if line[pos.0] == Tile::Wall {
										pos.1 -= 1;
										break;
									}
								}
							}
							Facing::Left => {
								for (i, tile) in map[pos.1].iter().enumerate().rev() {
									if *tile == Tile::Path {
										pos = (i, pos.1);
										break;
									} else if *tile == Tile::Wall {
										pos.0 += 1;
										break;
									}
								}
							}
							Facing::Up => {
								for (j, line) in map.iter().enumerate().rev() {
									if line[pos.0] == Tile::Path {
										pos = (pos.0, j);
										break;
									} else if line[pos.0] == Tile::Wall {
										pos.1 += 1;
										break;
									}
								}
							}
						},
					}
				}
			}
		}
	}

	println!("1: {}", 1000 * pos.1 + 4 * pos.0 + (facing as usize));

	Ok(())
}
