// Run in browser devtools on the input page
// (https://adventofcode.com/2022/day/3/input)

const input = document.getElementsByTagName("pre")[0].textContent;
const items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");
const lines = input.split("\n");

const priority = (a) => items.indexOf(a) + 1;

const shared = (a, b) => {
	for (const item of items) {
		if (a.includes(item) && b.includes(item)) {
			return item;
		}
	}
};

const common_group = (a, b, c) => {
	for (const item of items) {
		if (a.includes(item) && b.includes(item) && c.includes(item)) {
			return item;
		}
	}
};

const compartments = lines.map((s) => [
	s.slice(0, s.length / 2),
	s.slice(s.length / 2),
]);

const common_compartment = compartments.map((a) => shared(...a));
const res_1 = common_compartment.map(priority).reduce((a, b) => a + b);

console.log(`1: ${res_1}`);

let i = 0;
let groups = [];
while (i < lines.length) {
	groups.push([lines[i], lines[i + 1], lines[i + 2]]);
	i += 3;
}

const common_items = groups.map((g) => common_group(...g));
const res_2 = common_items.map(priority).reduce((a, b) => a + b);

console.log(`2: ${res_2}`);
