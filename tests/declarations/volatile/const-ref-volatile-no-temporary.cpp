// tidal-rules: VOL-10
// tidal-expect: reject
// tidal-error-clang: cannot bind to a temporary
// tidal-error-gcc: cannot bind lvalue reference of type .const volatile int&. to an rvalue
// tidal-error-edg: reference to const volatile must be an lvalue
const volatile int& r = 5;
