use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::CHAMBER_WIDTH;

pub const ROCK_HEIGHT: usize = 4;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rock {
	Falling,
	Stopped,
	None,
}

impl Display for Rock {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Rock::None => f.write_str("."),
			Rock::Falling => f.write_str("@"),
			Rock::Stopped => f.write_str("#"),
		}
	}
}

macro_rules! rock {
	($n:expr, $m:expr, $p:expr => [$($a:literal),*$(,)?]) => {{
		const N: usize = $n;
		const M: usize = $m;
		const P: usize = $p;

		let a = [$({
			let s: &str = $a;
			let mut arr = [$crate::Rock::None; N];
			let s = s.as_bytes();
			let mut i = 0;
			while i < s.len() {
				let c = match s[i] {
					b'#' | b'@' => $crate::Rock::Falling,
					b'.' | b' ' => $crate::Rock::None,
					_ => ::core::panic!("invalid Rock literal")
				};

				arr[i + P] = c;

				i += 1;
			}
			arr
		}),*];

		let mut arr = [[$crate::Rock::None; N]; M];

		let mut i = 0;
		while i < a.len() {
			arr[i + (arr.len() - a.len())] = a[i];
			i += 1;
		}
		arr
	}};
}

const ROCK_PADDING: usize = 2;
pub const ROCKS: [[[Rock; CHAMBER_WIDTH]; ROCK_HEIGHT]; 5] = [
	rock!(CHAMBER_WIDTH, ROCK_HEIGHT, ROCK_PADDING => [
		"@@@@",
	]),
	rock!(CHAMBER_WIDTH, ROCK_HEIGHT, ROCK_PADDING => [
		".@.",
		"@@@",
		".@.",
	]),
	rock!(CHAMBER_WIDTH, ROCK_HEIGHT, ROCK_PADDING => [
		"..@",
		"..@",
		"@@@",
	]),
	rock!(CHAMBER_WIDTH, ROCK_HEIGHT, ROCK_PADDING => [
		"@",
		"@",
		"@",
		"@",
	]),
	rock!(CHAMBER_WIDTH, ROCK_HEIGHT, ROCK_PADDING => [
		"@@",
		"@@",
	]),
];
