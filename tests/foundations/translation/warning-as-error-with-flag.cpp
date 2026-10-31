// tidal-rules: TR-6
// tidal-expect: reject
// tidal-error-clang: unused variable .unused
// tidal-error-gcc: unused variable .unused
// tidal-flags: -Wall -Werror
int f(int x) { int unused = x; return 0; }
