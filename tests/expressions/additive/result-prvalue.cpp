// tidal-rules: ADD-4 VC-12
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: must be an lvalue
int a = 1, b = 2;
int* p = &(a + b);
