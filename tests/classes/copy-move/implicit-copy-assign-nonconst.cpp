// tidal-rules: CPY-6
// tidal-expect: reject
struct M { M& operator=(M&) { return *this; } };
struct S { M m; };
void f(S& a, const S& b) { a = b; }
