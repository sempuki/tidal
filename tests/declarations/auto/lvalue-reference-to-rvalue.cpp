// tidal-rules: AUTO-18
// tidal-expect: reject
// tidal-error-clang: non-const lvalue reference .* cannot bind to a temporary
// tidal-error-gcc: cannot bind non-const lvalue reference
// tidal-error-edg: reference to non-const must be an lvalue
auto& r = 1;
