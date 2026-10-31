// tidal-rules: ODR-1 INC-4
// tidal-expect: reject
// tidal-error-clang: redefinition of .Outside
// tidal-error-gcc: redefinition of .struct Outside
// tidal-error-edg: class "Outside" has already been defined
#include "guard-text-after.h"
#include "guard-text-after.h"
int main() { return 0; }
