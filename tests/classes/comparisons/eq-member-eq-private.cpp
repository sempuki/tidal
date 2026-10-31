// tidal-rules: DCMP-2
// tidal-expect: reject
class M { bool operator==(const M&) const { return true; } };
struct S { M m; bool operator==(const S&) const = default; };
bool f(const S& a, const S& b) { return a == b; }
