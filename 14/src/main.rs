use std::{error::Error, fmt::Display, fs};

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("empty data")
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
	Empty,
	Wall,
	Sand,
}

#[allow(clippy::needless_range_loop)]
fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let mut map = vec![vec![Tile::Empty; 1000]; 1000];

	let mut max_y = 0;

	for line in input.lines() {
		let mut ox = 0usize;
		let mut oy = 0usize;

		for mut coords in line.split(" -> ").map(|s| s.split(',')) {
			let nx = coords.next().ok_or(EmptyError)?.parse()?;
			let ny = coords.next().ok_or(EmptyError)?.parse()?;
			max_y = max_y.max(ny);

			if ox == 0 && oy == 0 {
				(ox, oy) = (nx, ny);
				continue;
			}

			for i in ox.min(nx)..=nx.max(ox) {
				for j in oy.min(ny)..=ny.max(oy) {
					map[j][i] = Tile::Wall;
				}
			}

			(ox, oy) = (nx, ny);
		}
	}

	let mut n = 0;
	while sim_sand(&mut map, (500, 0)) {
		n += 1;
	}

	println!("1: {n}");

	for i in 0..map.len() {
		map[max_y + 2][i] = Tile::Wall;
	}

	while sim_sand(&mut map, (500, 0)) {
		n += 1;
	}

	println!("2: {n}");

	Ok(())
}

fn sim_sand(map: &mut Vec<Vec<Tile>>, source: (usize, usize)) -> bool {
	let (mut x, mut y) = source;

	if map[y][x] != Tile::Empty {
		return false;
	}

	while y + 1 < map.len() {
		if map[y + 1][x] == Tile::Empty {
			y += 1;
		} else if map[y + 1][x - 1] == Tile::Empty {
			y += 1;
			x -= 1;
		} else if map[y + 1][x + 1] == Tile::Empty {
			y += 1;
			x += 1;
		} else {
			map[y][x] = Tile::Sand;
			return true;
		}
	}

	false
}
