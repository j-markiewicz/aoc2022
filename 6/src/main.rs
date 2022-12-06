use std::{collections::HashSet, fs};

const SIZE_1: usize = 4;
const SIZE_2: usize = 14;

fn main() {
	let input = fs::read_to_string("./input.txt")
		.unwrap()
		.chars()
		.enumerate()
		.collect::<Vec<_>>();

	for window in input.windows(SIZE_1) {
		if HashSet::<char>::from_iter(window.iter().map(|d| d.1)).len() == SIZE_1 {
			println!("1: {}", window[SIZE_1 - 1].0 + 1);
			break;
		}
	}

	for window in input.windows(SIZE_2) {
		if HashSet::<char>::from_iter(window.iter().map(|d| d.1)).len() == SIZE_2 {
			println!("2: {}", window[SIZE_2 - 1].0 + 1);
			break;
		}
	}
}
