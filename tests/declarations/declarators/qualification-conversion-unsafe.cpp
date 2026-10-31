// tidal-rules: CV-7
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .const int \*\*. with an lvalue of type .int \*\*
// tidal-error-gcc: invalid conversion from .int\*\*. to .const int\*\*
// tidal-error-edg: value of type .int \*\*. cannot be used to initialize an entity of type .const int \*\*
int* p;
int** pp = &p;
const int** qq = pp;
