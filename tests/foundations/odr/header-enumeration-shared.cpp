// tidal-rules: ODR-7 INC-4
// tidal-expect: run
// tidal-sources: header-enumeration-shared.tu.cpp
#include "mode.h"
int main() { return next_code(Mode::busy) == static_cast<int>(Mode::done) ? 0 : 1; }
