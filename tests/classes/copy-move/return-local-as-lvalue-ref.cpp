// tidal-rules: CPY-11
// tidal-expect: reject
// tidal-error-clang: cannot bind to a temporary
// tidal-error-gcc: cannot bind non-const lvalue reference
// tidal-error-edg: reference to non-const must be an lvalue
int& f() { int x = 0; return x; }
int& g(int&& r) { return r; }
