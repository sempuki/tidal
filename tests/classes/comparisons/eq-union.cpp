// tidal-rules: DCMP-9
// tidal-expect: reject
// tidal-error-clang: implicitly deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
union U { int i; float f; bool operator==(const U&) const = default; };
bool f(const U& a, const U& b) { return a == b; }
