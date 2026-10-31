// tidal-rules: VOL-5
// tidal-expect: reject
// tidal-error-clang: volatile-qualified return type.*deprecated
// tidal-error-gcc: volatile.-qualified return type is deprecated
// tidal-flags: -Wall -Werror
volatile int f();
