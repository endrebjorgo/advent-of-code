#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>

#define BATTERY_BANK_CAPACITY 128
#define BATTERY_BANK_ARRAY_CAPACITY 1024

typedef struct {
    size_t batteries[BATTERY_BANK_CAPACITY];
    size_t size;
} BatteryBank;

BatteryBank bb_from_string(char *string) {
    BatteryBank bb = { .batteries = {0}, .size = 0 };
    size_t size = 0;
    while (1) {
        if (string[size] == '\n') break;
        bb.batteries[size] = (size_t)string[size] - 48;
        size++;
    }
    bb.size = size;
    return bb;
}

size_t bb_max_joltage(BatteryBank *bb, size_t on_count) {
    size_t max_joltage = 0;
    size_t max_index = 0;

    for (size_t i = 0; i < on_count; ++i) {
        for (size_t j = max_index + 1; j <= bb->size - on_count + i; ++j) {
            if (bb->batteries[j] > bb->batteries[max_index]) {
                max_index = j;
            }
        }
        max_joltage = max_joltage * 10 + bb->batteries[max_index];
        max_index++;
    }
    return max_joltage;
}

typedef struct {
    BatteryBank items[BATTERY_BANK_ARRAY_CAPACITY];
    size_t size;
} BatteryBankArray;

void bba_push_from_string(BatteryBankArray *bba, char *string) {
    assert(bba->size != BATTERY_BANK_ARRAY_CAPACITY);
    bba->items[bba->size] = bb_from_string(string);
    bba->size++;
}

BatteryBankArray bba_from_file(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file %s\n", file_path);
        exit(1);
    }

    BatteryBankArray bba = {
        .items = { { .batteries = {0}, .size = 0 } },
        .size = 0
    };


    char line[BATTERY_BANK_CAPACITY];
    while (fgets(line, BATTERY_BANK_CAPACITY, fp) != NULL) {
       bba_push_from_string(&bba, line);
    }
    return bba;
}

size_t bba_total_output_joltage(BatteryBankArray *bba, size_t on_count) {
    size_t total = 0;
    for (size_t i = 0; i < bba->size; ++i) {
        total += bb_max_joltage(&bba->items[i], on_count);
    }
    return total;
}

size_t part1(char *file_path) {
    BatteryBankArray bba = bba_from_file(file_path);
    return bba_total_output_joltage(&bba, 2);
}

size_t part2(char *file_path) {
    BatteryBankArray bba = bba_from_file(file_path);
    return bba_total_output_joltage(&bba, 12);
}

int main() {
    assert(part1("day03/test.txt") == 357);
    assert(part2("day03/test.txt") == 3121910778619);

    printf("Total output joltage (2): %zu\n", part1("day03/input.txt"));
    printf("Total output joltage (12): %zu\n", part2("day03/input.txt"));

    return 0;
}
