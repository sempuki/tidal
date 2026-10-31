// tidal-rules: ODR-8 INC-1
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of .twice\(int\)
// tidal-error-gcc: multiple definition of .twice\(int\)
// tidal-sources: header-function-in-two-units.tu.cpp
#include "twice.h"
int other();
int main() { return twice(1) + other() == 4 ? 0 : 1; }
