// tidal-rules: INC-4
// tidal-expect: run
// tidal-note: The test depends on #if evaluation and macro replacement, which the spec does not cover.
#include "guard-value.h"
#include "guard-value.h"
static_assert(TIDAL_ODR_GUARD_VALUE_H == 1);
int main() { Valued v{2}; return v.v - 2; }
