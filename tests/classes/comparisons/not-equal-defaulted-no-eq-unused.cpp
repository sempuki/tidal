// tidal-rules: DCMP-5
// tidal-expect: accept
struct S { int x; bool operator!=(const S&) const = default; };
