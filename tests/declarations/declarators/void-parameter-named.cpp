// tidal-rules: DCL-22
// tidal-expect: reject
// tidal-error-clang: argument may not have .void. type
// tidal-error-gcc: invalid use of type .void. in parameter declaration
// tidal-error-edg: parameter may not have void type
void f(void x);
