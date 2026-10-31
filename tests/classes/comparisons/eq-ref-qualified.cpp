// tidal-rules: DCMP-3
// tidal-expect: accept
struct S { int x; bool operator==(const S&) const& = default; };
