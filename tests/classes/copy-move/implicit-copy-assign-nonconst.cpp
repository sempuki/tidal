// tidal-rules: CPY-22
// tidal-expect: reject
// tidal-error-clang: no viable overloaded .=
// tidal-error-gcc: discards qualifiers
// tidal-error-edg: no operator .=. matches
struct M { M& operator=(M&) { return *this; } };
struct S { M m; };
void f(S& a, const S& b) { a = b; }
