// tidal-rules: VOL-10
// tidal-expect: reject
// tidal-error-clang: of type .int \*. with an rvalue of type .volatile int \*
// tidal-error-gcc: invalid conversion from .volatile int\*. to .int\*
// tidal-error-edg: volatile int \*. cannot be used to initialize an entity of type .int \*
volatile int v;
int* p = &v;
