// tidal-rules: CPY-7
// tidal-expect: reject
int g;
struct S { int& r = g; };
void f(S& a, const S& b) { a = b; }
