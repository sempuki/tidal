// tidal-rules: DCMP-2
// tidal-expect: reject
struct M {};
struct S { M m; bool operator==(const S&) const = default; };
bool f(const S& a, const S& b) { return a == b; }
