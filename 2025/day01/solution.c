#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

#define TURNS_ARRAY_BUF_SIZE 8192

typedef struct {
    int items[TURNS_ARRAY_BUF_SIZE];
    int size;
} TurnArray;

TurnArray turn_array_from_file(char *file_path) {
    TurnArray arr = {0};

    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file: %s\n", file_path);
        exit(1);
    }
    char line[64];
    
    while (fgets(line, sizeof(line), fp) != NULL) {
        if (arr.size == TURNS_ARRAY_BUF_SIZE) {
            fprintf(stderr, "ERROR: insufficient buffer size: %d\n", arr.size);
            exit(1);
        }
        int sign = (line[0] == 'L') ? -1 : 1;
        arr.items[arr.size] = atoi(line + 1) * sign;
        arr.size++;
    }
    return arr;
}

int modulo(int a, int b) {
    return ((a % b) + b) % b;
}

int part1(char *file_path) {
    TurnArray turn = turn_array_from_file(file_path);
    
    int count = 0;
    int dial_value = 50;
    const int dial_size = 100;
    
    for (int i = 0; i < turn.size; ++i) {
        dial_value = modulo(dial_value + turn.items[i], dial_size);
        if (dial_value == 0) {
            count++;
        }
    }
    return count;
}

int part2(char *file_path) {
    TurnArray turn = turn_array_from_file(file_path);

    int count = 0;
    int dial_value = 50;
    const int dial_size = 100;
    
    for (int i = 0; i < turn.size; ++i) {
        dial_value += turn.items[i];

        if (dial_value > 0) {
            count += dial_value / dial_size;
        } else {
            if (dial_value != turn.items[i]) count += 1;
            count -= dial_value / dial_size;
        }
        dial_value = modulo(dial_value, dial_size);
    }
    return count;
}

int main() {
    assert(part1("day01/test.txt") == 3);
    assert(part2("day01/test.txt") == 6);

    printf("Door password: %d\n", part1("day01/input.txt"));
    printf("Method 0x434C49434B: %d\n", part2("day01/input.txt"));

    return 0;
}
