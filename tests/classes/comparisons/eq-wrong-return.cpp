// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int x; int operator==(const S&) const = default; };
