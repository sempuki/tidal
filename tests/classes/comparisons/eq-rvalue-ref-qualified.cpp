// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int x; bool operator==(const S&) const&& = default; };
