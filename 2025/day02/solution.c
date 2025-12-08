#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>

typedef struct {
    size_t start;
    size_t end;
} IdRange;

typedef struct {
    IdRange items[64];
    size_t size;
} IdRangeArray;

IdRangeArray ira_from(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file %s\n", file_path);
        exit(1);
    }
    IdRangeArray ira = {};

    size_t a = 0;
    size_t b = 0;
    char c;
    while ((c = fgetc(fp)) != EOF) {
        if (c == '-') {
            a = b;
            b = 0;
        } else if (c == ',' || c == '\n') {
            ira.items[ira.size] = (IdRange) { .start = a, .end = b };
            ira.size++;
            if (c == '\n') break;
            a = 0;
            b = 0;
        } else {
            b = b * 10 + c - 48;
        }
    }
    return ira;
}


size_t count_digits(size_t n) {
    size_t count = 0;
    while (n > 0) {
        n /= 10;
        count++;
    }
    return count;
}

size_t power_of_ten(size_t e) {
    size_t result = 1;
    for (size_t i = 0; i < e; ++i) {
        result *= 10;
    }
    return result;
}

size_t part1(char *file_path) {
    IdRangeArray ira = ira_from(file_path);
    size_t invalid_id_sum = 0;

    size_t invalid_ids[1024];
    size_t invalid_ids_size = 0;

    for (size_t i = 0; i < ira.size; ++i) {
        size_t start = ira.items[i].start;
        size_t end = ira.items[i].end;
        size_t start_digits = count_digits(start);
        size_t end_digits = count_digits(end);

        for (size_t digits = start_digits; digits <= end_digits; ++digits) {
            if (digits % 2 != 0) continue;
            size_t seq_length = digits / 2;

            size_t multiplier = power_of_ten(seq_length);
            size_t increment = 1;
            for (size_t j = 1; j < digits / seq_length; ++j) {
                increment = increment * multiplier + 1;
            }
            size_t candidate = increment * multiplier / 10;
            for (size_t k = 0; k < 9 * multiplier / 10; ++k) {
                if (candidate > end) break;
                if (candidate >= start) {
                    int found_duplicate = 0;
                    for (size_t l = 0; l < invalid_ids_size; ++l) {
                        if (invalid_ids[l] == candidate) {
                            found_duplicate = 1;
                            break;
                        }
                    }
                    if (!found_duplicate) {
                        invalid_id_sum += candidate;
                        invalid_ids[invalid_ids_size] = candidate;
                        invalid_ids_size++;
                    }
                }
                candidate += increment;
            }
        }
    }
    return invalid_id_sum;
}

size_t part2(char *file_path) {
    IdRangeArray ira = ira_from(file_path);
    size_t invalid_id_sum = 0;

    size_t invalid_ids[1024];
    size_t invalid_ids_size = 0;

    for (size_t i = 0; i < ira.size; ++i) {

        size_t start = ira.items[i].start;
        size_t end = ira.items[i].end;
        size_t start_digits = count_digits(start);
        size_t end_digits = count_digits(end);

        for (size_t digits = start_digits; digits <= end_digits; ++digits) {
            for (size_t seq_length = 1; seq_length < digits; ++seq_length) {
                if (digits % seq_length != 0) continue;

                size_t multiplier = power_of_ten(seq_length);
                size_t increment = 1;
                for (size_t j = 1; j < digits / seq_length; ++j) {
                    increment = increment * multiplier + 1;
                }
                size_t candidate = increment * multiplier / 10;
                for (size_t k = 0; k < 9 * multiplier / 10; ++k) {
                    if (candidate > end) break;
                    if (candidate >= start) {
                        int found_duplicate = 0;
                        for (size_t l = 0; l < invalid_ids_size; ++l) {
                            if (invalid_ids[l] == candidate) {
                                found_duplicate = 1;
                                break;
                            }
                        }
                        if (!found_duplicate) {
                            invalid_id_sum += candidate;
                            invalid_ids[invalid_ids_size] = candidate;
                            invalid_ids_size++;
                        }
                    }
                    candidate += increment;
                }
            }
        }
    }
    return invalid_id_sum;
}

int main() {
    assert(part1("day02/test.txt") == 1227775554);
    assert(part2("day02/test.txt") == 4174379265);

    printf("Invalid ID sum: %zu\n", part1("day02/input.txt"));
    printf("Invalid ID sum (new rules): %zu\n", part2("day02/input.txt"));
    return 0;
}
