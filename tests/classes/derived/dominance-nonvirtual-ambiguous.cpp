// tidal-rules: DER-4
// tidal-expect: reject
// tidal-error-clang: found in multiple base
// tidal-error-gcc: request for member .* is ambiguous
// tidal-error-edg: is ambiguous
struct A { int x; };
struct B : A { int x; };
struct C : A {};
struct D : B, C {};
int f(D& d) { return d.x; }
