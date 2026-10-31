// tidal-rules: DCMP-2
// tidal-expect: reject
union U { int i; float f; bool operator==(const U&) const = default; };
bool f(const U& a, const U& b) { return a == b; }
