// tidal-rules: ODR-13 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: multiple definition of .rate<char>
// tidal-error-gcc: multiple definition of .rate<char>
// tidal-sources: explicit-variable-specialization-in-two-units.tu.cpp
#include "rate-template.h"
int main() { return rate<char> == 2 && rate_elsewhere() == 2 ? 0 : 1; }
