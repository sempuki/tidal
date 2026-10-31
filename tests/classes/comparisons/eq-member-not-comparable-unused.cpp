// tidal-rules: DCMP-2
// tidal-expect: accept
struct M {};
struct S { M m; bool operator==(const S&) const = default; };
