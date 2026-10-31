// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: invalid storage class specifier in function declarator
// tidal-error-gcc: storage class specified for parameter
// tidal-error-edg: invalid storage class for a parameter
void f(extern int x);
