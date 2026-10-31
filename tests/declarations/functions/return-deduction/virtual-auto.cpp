// tidal-rules: RET-5
// tidal-expect: reject
// tidal-error-clang: deduced return type cannot be virtual
// tidal-error-gcc: virtual function cannot have deduced return type
// tidal-error-edg: virtual function cannot have a deduced return type
// tidal-note: Gap review 2026-10-30.
struct S { virtual auto f() { return 1; } };
