// tidal-rules: ACC-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct B { void f(int) {} };
class D : public B { void f(double) {} };
void g(D& d) { d.f(1); }
