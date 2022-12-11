// gcc -std=c11 -Wall -o 11 11.c && ./11 && rm ./11

#include <stdint.h>
#include <string.h>
#include <stdbool.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

#define N_ROUNDS_1 20
#define N_ROUNDS_2 10000
#define N_MONKEYS 10
#define N_ITEMS 100
#define ITEM_OLD 0

typedef uint64_t Item;
typedef uint8_t MonkeyNumber;

struct Monkey {
	MonkeyNumber num;
	uint16_t n_items;
	Item* items;
	Item test_div;
	MonkeyNumber target_true;
	MonkeyNumber target_false;
	Item(*operation)(Item, Item);
	Item operand_a;
	Item operand_b;
	uint64_t items_inspected;
};

Item operation_add(Item a, Item b) {
	return a + b;
}

Item operation_mul(Item a, Item b) {
	return a * b;
}

struct Monkey* simulate(const struct Monkey*, uint8_t, uint16_t, bool);

int main() {
	struct Monkey* monkeys = calloc(N_MONKEYS, sizeof(struct Monkey));
	uint16_t n_monkeys;

	// Read and parse input
	FILE* file = fopen("input.txt", "rb");

	for (n_monkeys = 0; n_monkeys < N_MONKEYS; n_monkeys++) {
		struct Monkey monkey;
		monkey.items = calloc(N_ITEMS, sizeof(Item));
		char* item_list = malloc(N_ITEMS * 4);
		char* operand_a = malloc(128);
		char* operation = malloc(4);
		char* operand_b = malloc(128);

		if (fscanf(
			file,
			"Monkey %"SCNu8":\n"
			"  Starting items: %[0123456789, ]\n"
			"  Operation: new = %s %s %s\n"
			"  Test: divisible by %"SCNu64"\n"
			"    If true: throw to monkey %"SCNu8"\n"
			"    If false: throw to monkey %"SCNu8"\n"
			"\n",
			&monkey.num,
			item_list,
			operand_a, operation, operand_b,
			&monkey.test_div,
			&monkey.target_true,
			&monkey.target_false
		) != 8) {
			break;
		}

		if (monkey.num != n_monkeys) {
			break;
		}

		if (*operation == '*') {
			monkey.operation = operation_mul;
		} else if (*operation == '+') {
			monkey.operation = operation_add;
		} else {
			monkey.operation = NULL;
		}

		if (strcmp(operand_a, "old") == 0) {
			monkey.operand_a = ITEM_OLD;
		} else {
			monkey.operand_a = strtoul(operand_a, NULL, 10);
		}

		if (strcmp(operand_b, "old") == 0) {
			monkey.operand_b = ITEM_OLD;
		} else {
			monkey.operand_b = strtoul(operand_b, NULL, 10);
		}

		monkey.n_items = 1;
		uint16_t i = 0;
		bool expect_item = true;

		while (i < strlen(item_list)) {
			if (expect_item) {
				expect_item = false;
				monkey.items[monkey.n_items - 1] = strtoul(item_list + i, NULL, 10);
			}

			if (item_list[i] == ',') {
				monkey.n_items += 1;
				expect_item = true;
			}

			i++;
		}

		monkey.items_inspected = 0;

		monkeys[n_monkeys] = monkey;
	}

	fclose(file);

	// Simulate monkeys
	struct Monkey* monkeys_1 = simulate(monkeys, n_monkeys, N_ROUNDS_1, false);

	// Calculate monkey business
	uint64_t max = 0;
	uint64_t max2 = 0;

	for (uint16_t i = 0; i < n_monkeys; i++) {
		if (monkeys_1[i].items_inspected > max) {
			max2 = max;
			max = monkeys_1[i].items_inspected;
		} else if (monkeys_1[i].items_inspected > max2) {
			max2 = monkeys_1[i].items_inspected;
		}
	}

	printf("1: %"PRIu64"\n", max * max2);

	// Simulate monkeys, but this time with extra panic
	struct Monkey* monkeys_2 = simulate(monkeys, n_monkeys, N_ROUNDS_2, true);

	// Calculate monkey business
	uint64_t new_max = 0;
	uint64_t new_max2 = 0;

	for (uint16_t i = 0; i < n_monkeys; i++) {
		if (monkeys_2[i].items_inspected > new_max) {
			new_max2 = new_max;
			new_max = monkeys_2[i].items_inspected;
		} else if (monkeys_2[i].items_inspected > new_max2) {
			new_max2 = monkeys_2[i].items_inspected;
		}
	}

	printf("2: %"PRIu64"\n", new_max * new_max2);

	return 0;
}

struct Monkey* simulate(const struct Monkey* monkeys_in, uint8_t n_monkeys, uint16_t n_rounds, bool panic) {
	struct Monkey* monkeys = calloc(N_MONKEYS, sizeof(struct Monkey));
	memcpy(monkeys, monkeys_in, N_MONKEYS * sizeof(struct Monkey));

	for (uint16_t m = 0; m < n_monkeys; m++) {
		monkeys[m].items = calloc(N_ITEMS, sizeof(Item));
		memcpy(monkeys[m].items, monkeys_in[m].items, N_ITEMS * sizeof(Item));
	}

	Item mod = 1;
	for (uint16_t m = 0; m < n_monkeys; m++) {
		mod *= monkeys[m].test_div;
	}

	for (uint16_t i = 0; i < n_rounds; i++) {
		for (uint16_t j = 0; j < n_monkeys; j++) {
			struct Monkey* current = &monkeys[j];

			for (uint16_t k = 0; k < current->n_items; k++) {
				// Inspect item
				Item a = current->operand_a;
				Item b = current->operand_b;

				if (a == ITEM_OLD) {
					a = current->items[k];
				}

				if (b == ITEM_OLD) {
					b = current->items[k];
				}

				Item new = current->operation(a, b) % mod;
				current->items_inspected += 1;

				// Monkey gets bored
				if (!panic) {
					new /= 3;
				}

				// Test worry level
				bool test_res = new % current->test_div == 0;

				// Monkey throws item
				struct Monkey* target;
				if (test_res) {
					target = &monkeys[current->target_true];
				} else {
					target = &monkeys[current->target_false];
				}

				target->items[target->n_items] = new;
				target->n_items += 1;
			}

			current->n_items = 0;
		}
	}

	return monkeys;
}
