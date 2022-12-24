use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	fs,
};

const MAP_SIZE: usize = 256;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
	Empty,
	Elf,
}

type Dirs = [([(isize, isize); 3], (isize, isize)); 4];

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("empty data")
	}
}

macro_rules! add {
	($a:expr, $b:expr) => {{
		let a: (usize, usize) = $a;
		let b: (isize, isize) = $b;
		(
			a.0.checked_add_signed(b.0).unwrap(),
			a.1.checked_add_signed(b.1).unwrap(),
		)
	}};
}

fn sim(map: &mut [Vec<Tile>], dirs: &mut Dirs) -> bool {
	let mut props = Vec::new();

	for i in 0..MAP_SIZE {
		for j in 0..MAP_SIZE {
			if map[i][j] == Tile::Elf {
				let mut all_empty = true;
				'ae: for dir in *dirs {
					for d in dir.0 {
						let (x, y) = add!((j, i), d);
						if map[y][x] == Tile::Elf {
							all_empty = false;
							break 'ae;
						}
					}
				}

				if all_empty {
					continue;
				}

				for dir in *dirs {
					if dir.0.iter().all(|&p| {
						let (x, y) = add!((j, i), p);
						map[y][x] == Tile::Empty
					}) {
						props.push(((j, i), add!((j, i), dir.1)));
						break;
					}
				}
			}
		}
	}

	for prop in props.clone() {
		if props.iter().filter(|&&(_, p)| p == prop.1).count() > 1 {
			props.retain(|&(_, p)| p != prop.1);
		}
	}

	for (f, t) in &props {
		map[f.1][f.0] = Tile::Empty;
		map[t.1][t.0] = Tile::Elf;
	}

	*dirs = [dirs[1], dirs[2], dirs[3], dirs[0]];

	props.is_empty()
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let mut map = vec![vec![Tile::Empty; MAP_SIZE]; MAP_SIZE];
	for (i, line) in input.lines().enumerate() {
		for (j, char) in line.chars().enumerate() {
			if char == '#' {
				map[MAP_SIZE / 2 + i][MAP_SIZE / 2 + j] = Tile::Elf;
			}
		}
	}

	let mut dirs = [
		([(-1, -1), (0, -1), (1, -1)], (0, -1)),
		([(-1, 1), (0, 1), (1, 1)], (0, 1)),
		([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
		([(1, -1), (1, 0), (1, 1)], (1, 0)),
	];

	for _ in 0..10 {
		sim(&mut map, &mut dirs);
	}

	let min_x = map
		.iter()
		.filter_map(|r| {
			r.iter()
				.enumerate()
				.find_map(|(i, &t)| (t == Tile::Elf).then_some(i))
		})
		.min()
		.ok_or(EmptyError)?;
	let min_y = map
		.iter()
		.enumerate()
		.flat_map(|(i, r)| r.iter().map(move |&t| (i, t)))
		.find_map(|(i, t)| (t == Tile::Elf).then_some(i))
		.ok_or(EmptyError)?;
	let max_x = map
		.iter()
		.filter_map(|r| {
			r.iter()
				.enumerate()
				.rev()
				.find_map(|(i, &t)| (t == Tile::Elf).then_some(i))
		})
		.max()
		.ok_or(EmptyError)?;
	let max_y = map
		.iter()
		.enumerate()
		.rev()
		.flat_map(|(i, r)| r.iter().map(move |&t| (i, t)))
		.find_map(|(i, t)| (t == Tile::Elf).then_some(i))
		.ok_or(EmptyError)?;

	let mut count = 0;
	for row in map.iter().take(max_y + 1).skip(min_y) {
		for &tile in row.iter().take(max_x + 1).skip(min_x) {
			if tile == Tile::Empty {
				count += 1;
			}
		}
	}

	println!("1: {count}");

	let mut rounds = 10;

	loop {
		rounds += 1;

		if sim(&mut map, &mut dirs) {
			break;
		}
	}

	println!("2: {rounds}");

	Ok(())
}
