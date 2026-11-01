// tidal-rules: REF-6 REF-10
// tidal-expect: reject
// tidal-error-clang: could not bind to an lvalue
// tidal-error-gcc: cannot bind rvalue reference
// tidal-error-edg: rvalue reference cannot be bound to an lvalue
int store[2];
int (&&taken)[2]{store};
