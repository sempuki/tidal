// tidal-rules: RET-3
// tidal-expect: reject
// tidal-error-clang: for function with no return statements
// tidal-error-gcc: no return statements in function returning .auto&
// tidal-error-edg: cannot deduce .auto. type
// tidal-note: Gap review 2026-10-31.
auto& f() {}
