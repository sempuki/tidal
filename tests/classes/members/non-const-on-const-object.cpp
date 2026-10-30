// tidal-rules: CLS-5
// tidal-expect: reject
struct S { void change() {} };
void f(const S& s) { s.change(); }
