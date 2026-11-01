// tidal-rules: INC-4
// tidal-expect: run
// tidal-note: The test depends on #if evaluation and macro replacement, which the spec does not cover.
#include "guard-nested.h"
#include "guard-nested.h"
int main() { Nested n; return n.v - 4; }
