#include "limit-template.h"
const int* lower_elsewhere() { return &lower<int>; }
const int* upper_elsewhere() { return &upper<int>; }
