#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>

#define RANGES_CAPACITY 256
#define ITEMS_CAPACITY 1024

typedef struct {
    size_t beg;
    size_t end;
} IdRange;

int id_range_merge(IdRange *range, IdRange *other) {
    if (range->beg > other->end){
        return 0;
    }
    if (range->end < other->beg) {
        return 0;
    }
    if (range->beg > other->beg) {
        range->beg = other->beg;
    }
    if (range->end < other->end) {
        range->end = other->end;
    }
    return 1;
}

typedef struct {
    IdRange ranges[RANGES_CAPACITY];
    size_t  items[ITEMS_CAPACITY];
    size_t  ranges_size;
    size_t  items_size;
} Inventory;

Inventory inventory_from_file(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file %s\n", file_path);
        exit(1);
    }

    Inventory inventory = {
        .ranges = {{0}},
        .items = {0},
        .ranges_size = 0,
        .items_size = 0
    };

    size_t ranges_size = 0;
    size_t items_size = 0;

    char c;
    size_t a = 0;
    size_t b = 0;

    while ((c = fgetc(fp)) != EOF) {
        if (c == '\n') {
            if (b == 0) break;
            assert(a <= b);
            inventory.ranges[ranges_size] = (IdRange) { .beg = a, .end = b };
            ranges_size++;
            a = 0;
            b = 0;
        } else if (c == '-') {
            a = b;
            b = 0;
        } else {
            b = b * 10 + c - 48;
        }
    }
    inventory.ranges_size = ranges_size;

    while ((c = fgetc(fp)) != EOF) {
        if (c == '\n') {
            inventory.items[items_size] = b;
            items_size++;
            b = 0;
        } else {
            b = b * 10 + c - 48;
        }
    }
    inventory.items_size = items_size;
    return inventory;
}

size_t inventory_count_items_in_ranges(Inventory *inventory) {
    size_t count = 0;
    size_t curr_item;
    size_t found;
    IdRange curr_range;
    for (size_t i = 0; i < inventory->items_size; ++i) {
        found = 0;
        curr_item = inventory->items[i];
        for (size_t j = 0; j < inventory->ranges_size; ++j) {
            curr_range = inventory->ranges[j];
            if (curr_item >= curr_range.beg && curr_item <= curr_range.end) {
                count++;
                found = 1;
                break;
            } 
            if (found) break;
        }
    }
    return count;
}

void inventory_consolidate_ranges(Inventory *inventory) {
    IdRange curr_range;

    size_t found;
    IdRange new_ranges[RANGES_CAPACITY] = {{0}};
    size_t new_ranges_size;

    size_t consolidating = 1;

    while (consolidating) {
        consolidating = 0;

        new_ranges[0] = inventory->ranges[0];
        new_ranges_size = 1;

        for (size_t i = 1; i < inventory->ranges_size; ++i) {
            found = 0;

            curr_range = inventory->ranges[i];
            for (size_t j = 0; j < new_ranges_size; ++j) {
                if (!id_range_merge(&new_ranges[j], &curr_range)) {
                    continue;
                }
                consolidating = 1;
                found = 1;
                break;
            }
            if (!found) {
                new_ranges[new_ranges_size] = curr_range;
                new_ranges_size++;
            }
        }
        for (size_t i = 0; i < new_ranges_size; ++i) {
            inventory->ranges[i] = new_ranges[i];
            new_ranges[i] = (IdRange) {0};
        }
        inventory->ranges_size = new_ranges_size;
    }
}

size_t inventory_count_valid_ids(Inventory *inventory) {
    size_t count = 0;
    for (size_t i = 0; i < inventory->ranges_size; ++i) {
        count += inventory->ranges[i].end - inventory->ranges[i].beg + 1;
    }
    return count;
}

size_t part1(char *file_path) {
    Inventory inventory = inventory_from_file(file_path);
    return inventory_count_items_in_ranges(&inventory);
}

size_t part2(char *file_path) {
    Inventory inventory = inventory_from_file(file_path);
    inventory_consolidate_ranges(&inventory);
    return inventory_count_valid_ids(&inventory);
}

int main() {
    assert(part1("day05/test.txt") == 3);
    assert(part2("day05/test.txt") == 14);

    printf("Fresh ingredients: %zu\n", part1("day05/input.txt"));
    printf("Possible IDs: %zu\n", part2("day05/input.txt"));

    return 0;
}
