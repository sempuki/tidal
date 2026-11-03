// tidal-rules: DER-22
// tidal-expect: reject
// tidal-error-clang: found in multiple base
// tidal-error-gcc: request for member .* is ambiguous
// tidal-error-edg: is ambiguous
// tidal-note: The use is a call of a member function of the base.
struct A { int get() const { return 1; } };
struct B : A {};
struct D : B, A {};
int f(const D& d) { return d.get(); }
