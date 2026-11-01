// Deliberate mistake: name a helper that only stats.cpp declares (NAM-8).
#include "stats.h"
int main() { return stats::divide_down(10, 2) - 5; }
