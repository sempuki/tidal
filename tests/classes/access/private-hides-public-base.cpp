// tidal-rules: ACC-3
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// tidal-note: Gap review 2026-10-31.
struct B { void f(int) {} };
class D : public B { void f(double) {} };
void g(D& d) { d.f(1); }
