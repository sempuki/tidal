// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int x; friend bool operator==(const S&, S) = default; };
