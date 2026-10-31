// tidal-rules: REF-1
// tidal-expect: reject
// tidal-error-clang: cannot bind to a value of unrelated type
// tidal-error-gcc: cannot bind non-const lvalue reference of type .int&. to a value of type .long
// tidal-error-edg: not const-qualified\) cannot be initialized with a value of type .long
long l = 1;
int& r = l;
