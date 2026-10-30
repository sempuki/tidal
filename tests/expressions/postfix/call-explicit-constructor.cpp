// tidal-rules: PFX-6
// tidal-expect: reject
struct S { explicit S(int) {} };
void f(S);
void g() { f(1); }
