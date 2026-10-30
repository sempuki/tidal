// tidal-rules: CPY-3
// tidal-expect: reject
struct S { void operator=(const S&) = default; };
void f(S& a, const S& b) { a = b; }
