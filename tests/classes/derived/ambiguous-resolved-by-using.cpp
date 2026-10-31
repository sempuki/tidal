// tidal-rules: DER-4
// tidal-expect: accept
struct A { void f(int) {} };
struct B { void f(double) {} };
struct D : A, B { using A::f; using B::f; };
void g(D& d) { d.f(1); d.f(1.0); }
