// tidal-rules: SHF-4
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: lvalue required as unary
// tidal-error-edg: must be an lvalue
int a = 1;
int* p = &(a << 1);
