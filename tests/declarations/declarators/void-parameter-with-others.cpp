// tidal-rules: DCL-9
// tidal-expect: reject
// tidal-error-clang: must be the first and only parameter
// tidal-error-gcc: invalid use of type .void. in parameter declaration
// tidal-error-edg: parameter may not have void type
void f(void, int);
