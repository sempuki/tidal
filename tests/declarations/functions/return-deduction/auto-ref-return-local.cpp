// tidal-rules: RET-3
// tidal-expect: reject
// tidal-error-clang: non-const lvalue reference .* cannot bind to a temporary
// tidal-error-gcc: cannot bind non-const lvalue reference
// tidal-error-edg: reference to non-const must be an lvalue
// tidal-note: Gap review 2026-10-31.
auto& f() { int x = 1; return x; }
