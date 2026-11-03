// tidal-rules: INC-2
// tidal-expect: run
// tidal-note: The header in nested/ includes "../level.h". The name resolves from the directory of that header, so it names the level.h next to this file, which sets level to 1.
#include "nested/parent.h"
static_assert(parent_level == 1);
int main() { return level - 1; }
