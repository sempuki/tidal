// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-error-clang: volatile-qualified parameter type.*deprecated
// tidal-error-gcc: volatile.-qualified parameter is deprecated
// tidal-flags: -Wall -Werror
void f(volatile int x);
