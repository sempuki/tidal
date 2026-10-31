// tidal-rules: CV-7
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .int \*. with an rvalue of type .const int \*
// tidal-error-gcc: invalid conversion from .const int\*. to .int\*
// tidal-error-edg: value of type .const int \*. cannot be used to initialize an entity of type .int \*
const int x = 1;
int* p = &x;
