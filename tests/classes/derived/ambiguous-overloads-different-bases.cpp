// tidal-rules: DER-4
// tidal-expect: reject
// tidal-error-clang: found in multiple base
// tidal-error-gcc: request for member .* is ambiguous
// tidal-error-edg: is ambiguous
struct A { void f(int); };
struct B { void f(double); };
struct D : A, B {};
void g(D& d) { d.f(1); }
