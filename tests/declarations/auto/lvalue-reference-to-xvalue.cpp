// tidal-rules: AUTO-18
// tidal-expect: reject
// tidal-error-clang: non-const lvalue reference .* cannot bind to a temporary
// tidal-error-gcc: cannot bind non-const lvalue reference
// tidal-error-edg: reference to non-const must be an lvalue
// tidal-note: The initializer is an xvalue of type int, which is not const.
int i = 0;
auto& r = static_cast<int&&>(i);
