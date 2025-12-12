#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <assert.h>

#define OPERANDS_CAPACITY 8
#define PROBLEMS_CAPACITY 1024
#define STR_BUFFER_CAPACITY 4096

typedef enum {
    NONE = 0,
    ADD,
    MUL
} Operator;

Operator operator_from_char(char c) {
    switch (c) {
        case '+':
            return ADD;
        case '*':
            return MUL;
        default:
            fprintf(stderr, "ERROR: unreachable state\n");
            exit(1);
    }
}

typedef struct {
    size_t operands[OPERANDS_CAPACITY];
    size_t operands_count;
    Operator operator;
} Problem;

void problem_add_operand(Problem *problem, size_t operand) {
    problem->operands[problem->operands_count++] = operand;
}

void problem_set_operator(Problem *problem, Operator operator) {
    problem->operator = operator;
}

size_t problem_calculate_answer(Problem *problem) {
    size_t answer = 0;
    switch (problem->operator) {
        case ADD:
            for (size_t i = 0; i < problem->operands_count; ++i) {
                answer += problem->operands[i];
            }
            break;
        case MUL:
            answer = 1;
            for (size_t i = 0; i < problem->operands_count; ++i) {
                answer *= problem->operands[i];
            }
            break;
        default:
            fprintf(stderr, "ERROR: unreachable state\n");
            exit(1);
    }
    return answer;
}

void problem_print(Problem *problem) {
    char op = problem->operator == ADD ? '+' : '*';
    printf("%zu", problem->operands[0]);
    for (size_t i = 1; i < problem->operands_count; ++i) {
        printf(" %c %zu", op, problem->operands[i]);
    }
    printf("\n");
}

typedef struct {
    Problem problems[PROBLEMS_CAPACITY]; 
    size_t problems_count;
} ProblemsList;

int char_is_numeric(char c) {
    return (c >= 48 && c <= 57);
}
int char_is_operator(char c) {
    return (c == 42 || c == 43);
}

ProblemsList problems_list_from_file(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file %s\n", file_path);
        exit(1);
    }

    ProblemsList problems_list = { 
        .problems = { { .operands = {0}, .operands_count = 0, .operator = 0 } }, 
        .problems_count = 0 
    };

    char c;
    size_t curr_operand = 0;
    size_t problems_count = 0;
    size_t operands_count = 0;
    
    while ((c = fgetc(fp)) != EOF) {
        if (c == ' ') {
            if (curr_operand == 0) {
                continue;
            }
            problems_list.problems[problems_count].operands[operands_count] = curr_operand;
            problems_count++;
            curr_operand = 0;
        } else if (c == '\n') {
            if (curr_operand != 0) {
                problems_list.problems[problems_count].operands[operands_count] = curr_operand;
                problems_count++;
            } 
            assert(problems_list.problems_count == problems_count || problems_list.problems_count == 0);
            problems_list.problems_count = problems_count;
            curr_operand = 0;
            problems_count = 0;
            operands_count++;
        } else if (char_is_numeric(c)) {
            curr_operand = curr_operand * 10 + c - 48;
        } else if (char_is_operator(c)) {
            problems_list.problems[problems_count].operands_count = operands_count;
            problems_list.problems[problems_count].operator = operator_from_char(c);
            problems_count++;  
        } else {
            fprintf(stderr, "ERROR: unreachable state\n");
            exit(1);
        }
    }
    return problems_list;
}

void problems_list_print(ProblemsList *problems_list) {
    for (size_t i = 0; i < problems_list->problems_count; ++i) {
        problem_print(&problems_list->problems[i]);
    }
}

ProblemsList problems_list_from_file_cephalopod_math(char *file_path) {
    FILE *fp = fopen(file_path, "r");
    if (fp == NULL) {
        fprintf(stderr, "ERROR: unable to read file %s\n", file_path);
        exit(1);
    }

    ProblemsList problems_list = { 
        .problems = { { .operands = {0}, .operands_count = 0, .operator = 0 } }, 
        .problems_count = 0 
    };

    char lines[OPERANDS_CAPACITY][STR_BUFFER_CAPACITY];
    size_t lines_count = 0;

    while (fgets(lines[lines_count], STR_BUFFER_CAPACITY, fp)) {
        lines_count++;
    }
    
    char curr_operator;
    size_t curr_operand;
    Problem *curr_problem;

    size_t j = 0;
    while ((curr_operator = lines[lines_count - 1][j]) != '\n') {
        if (!char_is_operator(curr_operator)) {
            j++;
            continue;
        }
        curr_problem = &problems_list.problems[problems_list.problems_count++];
        problem_set_operator(curr_problem, operator_from_char(curr_operator));

        int parsing = 1;
        while (parsing && (lines[lines_count - 1][j]) != '\n') {
            curr_operand = 0;
            parsing = 0;
            for (size_t i = 0; i < lines_count - 1; ++i) {
                if (curr_operand != 0 && lines[i][j] == ' ') break;
                if (char_is_numeric(lines[i][j])) {
                    curr_operand = curr_operand * 10 + lines[i][j] - 48;
                    parsing = 1;
                }
            }
            if (curr_operand != 0) {
                problem_add_operand(curr_problem, curr_operand);
            }
            j++;
        }
    }
    return problems_list;
}

size_t problems_list_calculate_grand_total(ProblemsList *problems_list) {
    size_t grand_total = 0;
    for (size_t i = 0; i < problems_list->problems_count; ++i) {
        grand_total += problem_calculate_answer(&problems_list->problems[i]);
    }
    return grand_total;
}

size_t part1(char *file_path) {
    ProblemsList problems_list = problems_list_from_file(file_path);
    return problems_list_calculate_grand_total(&problems_list);
}

size_t part2(char *file_path) {
    ProblemsList problems_list = problems_list_from_file_cephalopod_math(file_path);
    return problems_list_calculate_grand_total(&problems_list);
}

int main() {
    assert(part1("day06/test.txt") == 4277556);
    assert(part2("day06/test.txt") == 3263827);

    printf("Grand total: %zu\n", part1("day06/input.txt"));
    printf("Grand total (cephalopod math): %zu\n", part2("day06/input.txt"));

    return 0;
}
