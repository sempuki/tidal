// tidal-rules: DER-4
// tidal-expect: reject
struct A { void f(int); };
struct B { void f(double); };
struct D : A, B {};
void g(D& d) { d.f(1); }
