// tidal-rules: DCMP-2
// tidal-expect: reject
struct S { union { int i; float f; }; bool operator==(const S&) const = default; };
bool f(const S& a, const S& b) { return a == b; }
