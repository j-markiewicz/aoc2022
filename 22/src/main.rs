#[path = "part-1.rs"]
mod part_1;
#[path = "part-2.rs"]
mod part_2;

use std::{
	env,
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

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;
	let print_map = env::args().nth(1).map(|s| s == "-p").unwrap_or_default();

	part_1::solve(&input, print_map)?;
	part_2::solve(&input, print_map)?;

	Ok(())
}
