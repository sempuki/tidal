// tidal-rules: DER-3
// tidal-expect: reject
struct B { void f(int); };
struct D : B { void f(const char*); };
void g(D& d) { d.f(1); }
