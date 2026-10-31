// tidal-rules: INC-3
// tidal-expect: reject
// tidal-error-clang: file not found
// tidal-error-gcc: no such file or directory
// tidal-error-edg: cannot open source file
#include "no-such-header.h"
int main() { return 0; }
