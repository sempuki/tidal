// tidal-rules: INC-2 INC-1
// tidal-expect: run
#include "nested/outer.h"
static_assert(outer_level == 2);
int main() { return level - 2; }
