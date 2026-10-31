// tidal-rules: INIT-4
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .int \*. with an rvalue of type .int
// tidal-error-gcc: invalid conversion from .int. to .int\*
// tidal-error-edg: value of type .int. cannot be used to initialize an entity of type .int \*
int* p = 1;
