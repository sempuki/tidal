// tidal-rules: ODR-1 INC-1
// tidal-expect: reject
// tidal-error-clang: redefinition of .counter
// tidal-error-gcc: redefinition of .int counter
// tidal-error-edg: has already been defined
#include "counter.h"
#include "counter.h"
int main() { return counter; }
