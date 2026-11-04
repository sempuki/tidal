// tidal-rules: CONV-18
// tidal-expect: reject
// tidal-error-clang: int \*. to .long|type .int \*
// tidal-error-gcc: invalid conversion from .int\*. to .long
// tidal-error-edg: type "int \*" cannot be used to initialize
int value = 7;
int* where = &value;
long n = where;
