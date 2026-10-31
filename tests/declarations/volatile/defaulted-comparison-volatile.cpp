// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int v; bool operator==(const S&) const volatile = default; };
