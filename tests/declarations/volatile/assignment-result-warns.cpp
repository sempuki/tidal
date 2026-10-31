// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-error-clang: result of assignment to object of volatile-qualified type.*deprecated
// tidal-error-gcc: value of assignment with .volatile.-qualified left operand is deprecated
// tidal-flags: -Wall -Werror
volatile int v;
int f() { int a = (v = 1); return a; }
