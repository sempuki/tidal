// tidal-rules: DER-22
// tidal-expect: reject
// tidal-error-clang: found in multiple base
// tidal-error-gcc: request for member .* is ambiguous
// tidal-error-edg: is ambiguous
struct A { int v; };
struct B : A {};
struct D : A, B {};
int f(D& d) { return d.v; }
