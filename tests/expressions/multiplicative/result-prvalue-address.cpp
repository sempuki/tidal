// tidal-rules: MUL-19
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: must be an lvalue
int a = 2, b = 3;
int* p = &(a * b);
