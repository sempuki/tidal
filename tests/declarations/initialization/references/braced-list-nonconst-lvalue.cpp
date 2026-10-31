// tidal-rules: REF-9
// tidal-expect: reject
// tidal-error-clang: cannot bind to an initializer list temporary
// tidal-error-gcc: cannot bind non-const lvalue reference
// tidal-error-edg: initial value of reference to non-const must be an lvalue
struct Point { int x, y; };
Point& p{1, 2};
