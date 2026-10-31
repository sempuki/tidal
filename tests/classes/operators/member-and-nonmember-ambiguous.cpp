// tidal-rules: OPR-15
// tidal-expect: reject
// tidal-error-clang: use of overloaded operator .\+. is ambiguous
// tidal-error-gcc: ambiguous overload for .operator\+
// tidal-error-edg: more than one operator .\+. matches
struct V { int x;  V operator+(V) const; };
V operator+(V, V);
V f(V a, V b) { return a + b; }
