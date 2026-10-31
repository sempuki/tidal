// tidal-rules: RET-3
// tidal-expect: reject
// tidal-error-clang: cannot be used before it is defined
// tidal-error-gcc: before deduction of .auto
// tidal-error-edg: cannot deduce the return type of function
auto f();
int x = f();
auto f() { return 1; }
