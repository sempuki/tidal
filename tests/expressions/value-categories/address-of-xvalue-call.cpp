// tidal-rules: VC-13
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: taking address of rvalue
// tidal-error-edg: must be an lvalue
// tidal-note: The xvalue is a call of a function that returns an rvalue reference.
int value = 0;
int&& give() { return static_cast<int&&>(value); }
int* p = &give();
