#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>

#define PAPER_ROLLS_CAPACITY 32768

typedef struct {
    int paper_rolls[PAPER_ROLLS_CAPACITY];
    int height;
    int width;
} PaperRollMap;

PaperRollMap prm_from_file(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR_ unable to read file %s\n", file_path);
        exit(1);
    }

    PaperRollMap prm = { .paper_rolls = {0}, .height = 0, .width = 0 };

    int size = 0;
    int height = 0;
    char c;

    while ((c = fgetc(fp)) != EOF) {
        switch (c) {
            case '@':
                prm.paper_rolls[size] = 1;
                size++;
                break;
            case '.':
                prm.paper_rolls[size] = 0;
                size++;
                break;
            case '\n':
                height++;
                break;
            default:
                fprintf(stderr, "ERROR: unreachable state\n");
                exit(1);
                break;
        }
    }
    assert(size % height == 0);
    prm.height = height;
    prm.width = size / height;
    return prm;
}

void prm_print(PaperRollMap *prm) {
    printf("heigth=%d\n", prm->height);
    printf("width=%d\n", prm->width);
    for (int i = 0; i < prm->height; ++i) {
        for (int j = 0; j < prm->width; ++j) {
            printf("%d ", prm->paper_rolls[i * prm->width + j]);
        }
        printf("\n");
    }
}

int prm_count_neighbors(PaperRollMap *prm, int i, int j) {
    int neighbors = 0;
    for (int i_offset = -1; i_offset <= 1; ++i_offset) {
        for (int j_offset = -1; j_offset <= 1; ++j_offset) {
            if (i_offset == 0 && j_offset == 0) {
                continue;
            }
            int new_i = i + i_offset;
            int new_j = j + j_offset;
            if (new_i < 0 || new_i >= prm->width) {
                continue;
            }
            if (new_j < 0 || new_j >= prm->height) {
                continue;
            }
            if (prm->paper_rolls[new_i * prm->width + new_j]) {
                neighbors++;
            }
        }
    }
    return neighbors;
}

int prm_total_accessible(PaperRollMap *prm) {
    int total_accessible = 0;
    for (int i = 0; i < prm->height; ++i) {
        for (int j = 0; j < prm->width; ++j) {
            if (prm->paper_rolls[i * prm->width + j] == 0) {
                continue;
            }
            if (prm_count_neighbors(prm, i, j) < 4) {
                total_accessible++;
            }
        }
    }
    return total_accessible;
}

int prm_total_removed(PaperRollMap prm) {
    int total_removed = 0;
    int searching = 1;
    while (searching) {
        searching = 0;
        for (int i = 0; i < prm.height; ++i) {
            for (int j = 0; j < prm.width; ++j) {
                if (prm.paper_rolls[i * prm.width + j] == 0) {
                    continue;
                }
                if (prm_count_neighbors(&prm, i, j) < 4) {
                    prm.paper_rolls[i * prm.width + j] = 0;
                    total_removed++;
                    searching = 1;
                }
            }
        }
    }
    return total_removed;
}

int part1(char *file_path) {
    PaperRollMap prm = prm_from_file(file_path);
    return prm_total_accessible(&prm);
}

int part2(char *file_path) {
    PaperRollMap prm = prm_from_file(file_path);
    return prm_total_removed(prm);
}

int main() {
    assert(part1("day04/test.txt") == 13);
    assert(part2("day04/test.txt") == 43);

    printf("Accessible rolls: %d\n", part1("day04/input.txt"));
    printf("Removable rolls: %d\n", part2("day04/input.txt"));

    return 0;
}
