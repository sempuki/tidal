// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: must be const-qualified
// tidal-error-gcc: must be .const
// tidal-error-edg: must be .const
struct S { int x; bool operator==(const S&) = default; };
