use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("./input.txt")?;

	let mut i = 0;
	let mut x = 1;
	let mut xs = Vec::new();
	let mut crt = Vec::new();

	for line in input.lines() {
		match &line[..4] {
			"noop" => {
				crt.push(i64::abs_diff(x, i % 40) <= 1);
				i += 1;
				xs.push(x * i);
			}
			"addx" => {
				crt.push(i64::abs_diff(x, i % 40) <= 1);
				i += 1;
				xs.push(x * i);
				crt.push(i64::abs_diff(x, i % 40) <= 1);
				i += 1;
				xs.push(x * i);
				x += line[5..].parse::<i64>()?;
			}
			_ => panic!(),
		}
	}

	let mut sum = 0;
	for i in 0..6 {
		sum += xs[19 + i * 40]
	}

	println!("1: {}", sum);

	let crt = crt
		.iter()
		.map(|&p| if p { '#' } else { '.' })
		.collect::<String>();
	println!("2: {}", &crt[0..40]);
	println!("   {}", &crt[40..80]);
	println!("   {}", &crt[80..120]);
	println!("   {}", &crt[120..160]);
	println!("   {}", &crt[160..200]);
	println!("   {}", &crt[200..240]);

	Ok(())
}
