// tidal-rules: ODR-10
// tidal-expect: run
// tidal-sources: const-value-in-inline-two-units.tu.cpp
// tidal-note: limit names a different object in each unit. The inline function only reads its value, so the two definitions of twice_limit match.
#include "limit-value.h"
int other();
int main() { return twice_limit() + other() == 32 ? 0 : 1; }
