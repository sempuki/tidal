// tidal-rules: VC-3
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: taking address of rvalue
// tidal-error-edg: must be an lvalue
int x = 0;
int* p = &static_cast<int&&>(x);
