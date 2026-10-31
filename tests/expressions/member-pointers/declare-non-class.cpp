// tidal-rules: MPTR-8
// tidal-expect: reject
// tidal-error-clang: does not point into a class
// tidal-error-gcc: expected id-expression before .\*. token
// tidal-error-edg: invalid combination of type specifiers
int int::* pm;
