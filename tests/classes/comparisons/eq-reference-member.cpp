// tidal-rules: DCMP-2
// tidal-expect: reject
struct S { int& r; bool operator==(const S&) const = default; };
bool f(const S& a, const S& b) { return a == b; }
