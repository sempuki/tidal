// tidal-rules: DCMP-3
// tidal-expect: reject
struct T {};
struct S { int x; bool operator==(const T&) const = default; };
