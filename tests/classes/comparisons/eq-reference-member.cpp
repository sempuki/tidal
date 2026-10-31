// tidal-rules: DCMP-2
// tidal-expect: reject
// tidal-error-clang: implicitly deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct S { int& r; bool operator==(const S&) const = default; };
bool f(const S& a, const S& b) { return a == b; }
