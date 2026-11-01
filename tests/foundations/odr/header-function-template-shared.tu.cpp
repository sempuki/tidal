#include "twice-template.h"
IntFunction twice_elsewhere() { return &twice<int>; }
int* tally_elsewhere() { return &tally<int>(); }
