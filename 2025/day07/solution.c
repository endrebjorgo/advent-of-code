#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>

#define MAX_HEIGHT 256
#define MAX_WIDTH  256
#define BYTES_COUNT 32
#define BITS_COUNT 8 * BYTES_COUNT

typedef struct {
    uint8_t bytes[BYTES_COUNT];
} Positions;

void positions_print(Positions *positions) {
    for (size_t i = 0; i < BYTES_COUNT; ++i) {
        for (size_t j = 0; j < 8; ++j) {
            putchar('0' + ((positions->bytes[BYTES_COUNT - i - 1] >> (7 - j)) & 1));
        }
    }
    printf("\n");
}

void positions_bitwise_and(Positions *positions, Positions *other) {
    for (size_t i = 0; i < BYTES_COUNT; ++i) {
        positions->bytes[i] &= other->bytes[i];
    }
}

void positions_bitwise_or(Positions *positions, Positions *other) {
    for (size_t i = 0; i < BYTES_COUNT; ++i) {
        positions->bytes[i] |= other->bytes[i];
    }
}

void positions_bitwise_xor(Positions *positions, Positions *other) {
    for (size_t i = 0; i < BYTES_COUNT; ++i) {
        positions->bytes[i] ^= other->bytes[i];
    }
}

void positions_flip_bit(Positions *positions, size_t idx) {
    size_t byte_idx = idx / 8;
    size_t bit_idx = idx % 8;
    positions->bytes[byte_idx] ^= (1 << bit_idx);
}

void positions_lsh(Positions *positions, size_t count) {
    for (size_t c = 0; c < count; ++c) {
        uint8_t prev_overflow = 0;
        uint8_t curr_overflow = 0;
        for (size_t i = 0; i < BYTES_COUNT; ++i) {
            prev_overflow = curr_overflow;
            curr_overflow = positions->bytes[i] & (1 << 7);
            positions->bytes[i] <<= 1;
            if (prev_overflow) {
                positions->bytes[i] |= 1;
            }
        }
    }
}

void positions_rsh(Positions *positions, size_t count) {
    for (size_t c = 0; c < count; ++c) {
        uint8_t prev_underflow = 0;
        uint8_t curr_underflow = 0;
        for (size_t i = 0; i < BYTES_COUNT; ++i) {
            prev_underflow = curr_underflow;
            curr_underflow = positions->bytes[BYTES_COUNT - i - 1] & 1;
            positions->bytes[BYTES_COUNT - i -1] >>= 1;
            if (prev_underflow) {
                positions->bytes[BYTES_COUNT - i - 1] |= (1 << 7);
            }
        }
    }
}

size_t positions_count_ones(Positions *positions) {
    size_t count = 0;
    uint8_t curr_byte;
    for (size_t i = 0; i < BYTES_COUNT; ++i) {
        curr_byte = positions->bytes[i];
        while (curr_byte != 0) {
            count += curr_byte & 1;
            curr_byte >>= 1;
        }
    }
    return count;
}

typedef struct {
    Positions beams;
    Positions rows[MAX_HEIGHT];
    size_t rows_count;
} Manifold;

Manifold manifold_from_file(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file %s\n", file_path);
        exit(1);
    }

    Manifold manifold = { 
        .beams = {0}, 
        .rows = {{0}}, 
        .rows_count = 0
    };

    size_t i = 0;
    char line_buffer[MAX_WIDTH];
    size_t line_contains_splitter;

    while (fgets(line_buffer, MAX_WIDTH, fp)) {
        i = 0;
        line_contains_splitter = 0;
        while (line_buffer[i] != '\n') {
            if (line_buffer[i] == 'S') {
                positions_flip_bit(&manifold.beams, i);
                break;
            }
            if (line_buffer[i] == '^') {
                line_contains_splitter = 1;
                positions_flip_bit(&manifold.rows[manifold.rows_count], i);
            }
            i++;
        }
        if (line_contains_splitter) manifold.rows_count++;
    }
    return manifold;
}

size_t manifold_count_splits(Manifold *manifold) {
    size_t count = 0;
    Positions beams = manifold->beams; // copy starting beams
    for (size_t i = 0; i < manifold->rows_count; ++i) {
        Positions temp  = {0};
        Positions unsplit  = {0};
        positions_bitwise_xor(&unsplit, &beams); // copy beams to unsplit
        positions_bitwise_and(&beams, &manifold->rows[i]); // beam intersects splitters
        positions_bitwise_xor(&unsplit, &beams); // remove intersections from unsplit
        count += positions_count_ones(&beams); // add intersections aka splits to count
        positions_bitwise_xor(&temp, &beams); // copy beams to temp
        positions_lsh(&beams, 1); // beams go left
        positions_rsh(&temp, 1); // beams go right
        positions_bitwise_or(&beams, &temp); // union of shifted beams
        positions_bitwise_or(&beams, &unsplit); // add unsplit 
    }
    return count;
}

size_t mct_recursive(Manifold *manifold, size_t idx, size_t row, size_t memo[][BITS_COUNT]) {
    if (row == manifold->rows_count) return 1;
    if (memo[idx][row] != 0) return memo[idx][row];

    size_t count = 0;

    Positions left = {0};
    Positions right = {0};
    Positions unsplit = {0};
    positions_flip_bit(&left, idx);
    
    positions_bitwise_xor(&unsplit, &left); 
    positions_bitwise_and(&left, &manifold->rows[row]); 
    positions_bitwise_xor(&unsplit, &left); 
    positions_bitwise_xor(&right, &left); 
    positions_lsh(&left, 1); 
    positions_rsh(&right, 1);
    
    size_t next_idx;
    size_t overlapping;
    for (size_t i = 0; i < BYTES_COUNT; ++i) {
        for (size_t j = 0; j < 8; ++j) {
            next_idx = 8 * i + j;
            overlapping = (unsplit.bytes[i] >> j) & 1;
            overlapping += (left.bytes[i] >> j) & 1;
            overlapping += (right.bytes[i] >> j) & 1;
            if (overlapping == 0) continue;
            count += overlapping * mct_recursive(manifold, next_idx, row + 1, memo);
        }
    }
    memo[idx][row] = count;
    return count;
}

size_t manifold_count_timelines(Manifold *manifold) {
    size_t start_idx = 0;

    size_t memo[BITS_COUNT][BITS_COUNT] = {{0}};

    while (start_idx < BYTES_COUNT * 8) {
        if (manifold->beams.bytes[0] & 1) break;
        positions_rsh(&manifold->beams, 1);
        start_idx++;
    }
    return mct_recursive(manifold, start_idx, 0, memo);
}

size_t part1(char *file_path) {
    Manifold manifold = manifold_from_file(file_path);
    return manifold_count_splits(&manifold);
}

size_t part2(char *file_path) {
    Manifold manifold = manifold_from_file(file_path);
    return manifold_count_timelines(&manifold);
}

int main() {
    assert(part1("day07/test.txt") == 21);
    assert(part2("day07/test.txt") == 40);

    printf("Total splits: %zu\n", part1("day07/input.txt"));
    printf("Total timelines: %zu\n", part2("day07/input.txt"));

    return 0;
}
