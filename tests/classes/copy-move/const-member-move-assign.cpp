// tidal-rules: CPY-7
// tidal-expect: reject
struct S { const int c = 1; };
void f(S& a, S& b) { a = static_cast<S&&>(b); }
