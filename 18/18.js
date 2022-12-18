// Run in browser devtools on the input page
// (https://adventofcode.com/2022/day/18/input)

const input = document.getElementsByTagName("pre")[0].textContent;
const lines = input.trim().split("\n");
const cubes = lines.map((t) => t.split(",").map((n) => parseInt(n, 10)));
let sides = cubes.length * 6;

for (cube_a of cubes) {
	for (cube_b of cubes) {
		if (
			(cube_a[0] == cube_b[0] &&
				cube_a[1] == cube_b[1] &&
				cube_a[2] - cube_b[2] == 1) ||
			(cube_a[1] == cube_b[1] &&
				cube_a[2] == cube_b[2] &&
				cube_a[0] - cube_b[0] == 1) ||
			(cube_a[2] == cube_b[2] &&
				cube_a[0] == cube_b[0] &&
				cube_a[1] - cube_b[1] == 1)
		) {
			sides -= 2;
		}
	}
}

console.log(`1: ${sides}`);

let not_cubes = [];
let outside_not_cubes = [];

for (
	let x = Math.min(...cubes.map((c) => c[0])) - 1;
	x < Math.max(...cubes.map((c) => c[0])) + 1;
	x += 1
) {
	for (
		let y = Math.min(...cubes.map((c) => c[1])) - 1;
		y < Math.max(...cubes.map((c) => c[1])) + 1;
		y += 1
	) {
		for (
			let z = Math.min(...cubes.map((c) => c[2])) - 1;
			z < Math.max(...cubes.map((c) => c[2])) + 1;
			z += 1
		) {
			outside_not_cubes.push([x, y, z]);
		}
	}
}

for (
	let x = Math.min(...cubes.map((c) => c[0]));
	x < Math.max(...cubes.map((c) => c[0])) + 1;
	x += 1
) {
	for (
		let y = Math.min(...cubes.map((c) => c[1]));
		y < Math.max(...cubes.map((c) => c[1])) + 1;
		y += 1
	) {
		for (
			let z = Math.min(...cubes.map((c) => c[2]));
			z < Math.max(...cubes.map((c) => c[2])) + 1;
			z += 1
		) {
			outside_not_cubes = outside_not_cubes.filter(
				(c) => JSON.stringify(c) != JSON.stringify([x, y, z])
			);

			if (!cubes.map(JSON.stringify).includes(JSON.stringify([x, y, z]))) {
				not_cubes.push([x, y, z]);
			}
		}
	}
}

for (let __i = 0; __i < 100; __i++) {
	const c_not_cubes = JSON.parse(JSON.stringify(not_cubes));
	const c_outside_not_cubes = JSON.parse(JSON.stringify(outside_not_cubes));

	for (cube_a of c_not_cubes) {
		for (cube_b of c_outside_not_cubes) {
			if (
				(cube_a[0] == cube_b[0] &&
					cube_a[1] == cube_b[1] &&
					Math.abs(cube_a[2] - cube_b[2]) == 1) ||
				(cube_a[1] == cube_b[1] &&
					cube_a[2] == cube_b[2] &&
					Math.abs(cube_a[0] - cube_b[0]) == 1) ||
				(cube_a[2] == cube_b[2] &&
					cube_a[0] == cube_b[0] &&
					Math.abs(cube_a[1] - cube_b[1]) == 1)
			) {
				if (
					!outside_not_cubes
						.map(JSON.stringify)
						.includes(JSON.stringify(cube_a))
				) {
					outside_not_cubes.push(cube_a);
				}

				not_cubes = not_cubes.filter(
					(c) => JSON.stringify(c) != JSON.stringify(cube_a)
				);
			}
		}
	}
}

let outsides = not_cubes.length * 6;

for (cube_a of not_cubes) {
	for (cube_b of not_cubes) {
		if (
			(cube_a[0] == cube_b[0] &&
				cube_a[1] == cube_b[1] &&
				cube_a[2] - cube_b[2] == 1) ||
			(cube_a[1] == cube_b[1] &&
				cube_a[2] == cube_b[2] &&
				cube_a[0] - cube_b[0] == 1) ||
			(cube_a[2] == cube_b[2] &&
				cube_a[0] == cube_b[0] &&
				cube_a[1] - cube_b[1] == 1)
		) {
			outsides -= 2;
		}
	}
}

console.log(`2: ${sides - outsides}`);
