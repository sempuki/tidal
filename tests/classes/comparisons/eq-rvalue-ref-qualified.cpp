// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: ref-qualifier .&&. is not allowed
// tidal-error-gcc: must not have .&&. ref-qualifier
// tidal-error-edg: cannot be .&&.-qualified
struct S { int x; bool operator==(const S&) const&& = default; };
