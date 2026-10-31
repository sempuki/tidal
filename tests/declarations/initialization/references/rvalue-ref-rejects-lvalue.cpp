// tidal-rules: REF-10
// tidal-expect: reject
// tidal-error-clang: rvalue reference to type .int. cannot bind to lvalue
// tidal-error-gcc: cannot bind rvalue reference of type .int&&. to lvalue
// tidal-error-edg: rvalue reference cannot be bound to an lvalue
int x = 1;
int&& r = x;
