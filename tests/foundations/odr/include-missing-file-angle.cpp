// tidal-rules: INC-3
// tidal-expect: reject
// tidal-error-clang: file not found
// tidal-error-gcc: no such file or directory
// tidal-error-edg: cannot open source file
// tidal-note: The directive uses angle brackets and a path with a directory.
#include <no-such-directory/none.h>
int main() { return 0; }
