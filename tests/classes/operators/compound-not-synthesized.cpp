// tidal-rules: OPR-15
// tidal-expect: reject
// tidal-error-clang: no viable overloaded .\+=
// tidal-error-gcc: no match for .operator\+=
// tidal-error-edg: no operator .\+=. matches
struct V { int x;  V operator+(V o) const { return {x + o.x}; } };
void f(V& a, V b) { a += b; }
