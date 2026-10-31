// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-error-clang: increment of object of volatile-qualified type.*deprecated
// tidal-error-gcc: \+\+. expression of .volatile.-qualified type is deprecated
// tidal-flags: -Wall -Werror
volatile int v;
void f() { v++; }
