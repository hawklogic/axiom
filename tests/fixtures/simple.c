// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

// Simple C test fixture for parser tests

#include <stdio.h>

#define MAX_VALUE 100

typedef struct {
    int x;
    int y;
} Point;

enum Status {
    OK = 0,
    ERROR = 1
};

int add(int a, int b) {
    return a + b;
}

int main(void) {
    Point p = {10, 20};
    int result = add(p.x, p.y);
    printf("Result: %d\n", result);
    return 0;
}
