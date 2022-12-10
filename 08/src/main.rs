use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let trees = input
		.lines()
		.map(|l| {
			l.chars()
				.map(|c| c.to_string().parse::<i8>())
				.collect::<Result<Vec<_>, _>>()
		})
		.collect::<Result<Vec<_>, _>>()?;

	let mut n_visible = 0;

	for i in 0..trees.len() {
		for j in 0..trees[0].len() {
			if trees[i][j] > *trees[i][j + 1..].iter().max().unwrap_or(&i8::MIN)
				|| trees[i][j] > *trees[i][..j].iter().max().unwrap_or(&i8::MIN)
				|| trees[i][j] > trees[i + 1..].iter().map(|t| t[j]).max().unwrap_or(i8::MIN)
				|| trees[i][j] > trees[..i].iter().map(|t| t[j]).max().unwrap_or(i8::MIN)
			{
				n_visible += 1;
			}
		}
	}

	println!("1: {n_visible}");

	let mut scores = vec![vec![0u64; trees[0].len()]; trees.len()];

	for i in 0..trees.len() {
		for j in 0..trees[0].len() {
			let mut score = 1;

			let mut s = 0;
			for tree in trees[i][j + 1..].iter().copied() {
				s += 1;

				if tree >= trees[i][j] {
					break;
				}
			}
			score *= s;

			let mut s = 0;
			for tree in trees[i][..j].iter().rev().copied() {
				s += 1;

				if tree >= trees[i][j] {
					break;
				}
			}
			score *= s;

			let mut s = 0;
			for tree in trees[i + 1..].iter().map(|t| t[j]) {
				s += 1;

				if tree >= trees[i][j] {
					break;
				}
			}
			score *= s;

			let mut s = 0;
			for tree in trees[..i].iter().rev().map(|t| t[j]) {
				s += 1;

				if tree >= trees[i][j] {
					break;
				}
			}
			score *= s;

			scores[i][j] = score;
		}
	}

	println!(
		"2: {}",
		scores.into_iter().flatten().max().unwrap_or(u64::MAX)
	);

	Ok(())
}
