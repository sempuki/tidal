// tidal-rules: DER-10
// tidal-expect: reject
// tidal-error-clang: cannot initialize return object of type .D \*. with an rvalue of type .B \*
// tidal-error-gcc: invalid conversion from .B\*. to .D\*
// tidal-error-edg: return value type does not match
struct B {};
struct D : B {};
D* f(B* p) { return p; }
