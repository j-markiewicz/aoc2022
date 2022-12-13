use std::{cmp::Ordering, error::Error, fmt::Display, fs};

use serde_json::Value;

const DIVIDER_1: &str = "[[2]]";
const DIVIDER_2: &str = "[[6]]";

#[derive(Debug)]
struct EmptyError;

impl Error for EmptyError {}

impl Display for EmptyError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("empty data")
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let mut correct = Vec::new();
	for (i, pair) in input.split("\n\n").enumerate() {
		let mut lines = pair.lines();
		let a = serde_json::from_str::<Value>(lines.next().ok_or(EmptyError)?)?;
		let b = serde_json::from_str::<Value>(lines.next().ok_or(EmptyError)?)?;

		if compare(a, b).is_lt() {
			correct.push(i + 1);
		}
	}

	println!("1: {}", correct.into_iter().sum::<usize>());

	let mut input = (input + "\n" + DIVIDER_1 + "\n" + DIVIDER_2 + "\n")
		.lines()
		.filter(|l| !l.trim().is_empty())
		.map(|l| l.trim().to_string())
		.collect::<Vec<_>>();

	input.sort_unstable_by(|a, b| compare_lines(a, b));

	let i = input
		.binary_search_by(|l| compare_lines(l, DIVIDER_1))
		.map_err(|_| EmptyError)?
		+ 1;
	let j = input
		.binary_search_by(|l| compare_lines(l, DIVIDER_2))
		.map_err(|_| EmptyError)?
		+ 1;

	println!("2: {}", i * j);

	Ok(())
}

fn compare_lines(a: impl AsRef<str>, b: impl AsRef<str>) -> Ordering {
	let a = serde_json::from_str::<Value>(a.as_ref()).expect("invalid format");
	let b = serde_json::from_str::<Value>(b.as_ref()).expect("invalid format");
	compare(a, b)
}

fn compare(a: Value, b: Value) -> Ordering {
	match (a, b) {
		(Value::Number(a), Value::Number(b)) => a.as_i64().cmp(&b.as_i64()),
		(Value::Array(a), Value::Array(b)) => {
			let mut a = a.into_iter();
			let mut b = b.into_iter();

			loop {
				match (a.next(), b.next()) {
					(None, None) => break Ordering::Equal,
					(None, Some(_)) => break Ordering::Less,
					(Some(_), None) => break Ordering::Greater,
					(Some(a), Some(b)) => {
						let ord = compare(a.clone(), b.clone());
						if !ord.is_eq() {
							break compare(a, b);
						}
					}
				}
			}
		}
		(a @ Value::Array(_), b @ Value::Number(_)) => compare(a, Value::Array(vec![b])),
		(a @ Value::Number(_), b @ Value::Array(_)) => compare(Value::Array(vec![a]), b),
		_ => panic!(),
	}
}
