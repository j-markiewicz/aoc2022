use std::fs;

fn main() {
	let mut food: Vec<u64> = fs::read_to_string("./input.txt")
		.unwrap()
		.split("\n\n")
		.map(|s| s.lines().map(|n| n.parse::<u64>().unwrap()).sum())
		.collect();

	food.sort_by(|a, b| b.cmp(a));

	println!("1: {}", food[0]);
	println!("2: {}", food[0..=2].iter().sum::<u64>());
}
