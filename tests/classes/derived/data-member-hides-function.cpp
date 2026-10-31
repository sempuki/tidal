// tidal-rules: DER-3
// tidal-expect: reject
// tidal-error-clang: is not a function or function pointer
// tidal-error-gcc: cannot be used as a function
// tidal-error-edg: must have \(pointer-to-\) function type
struct B { int f(); };
struct D : B { int f = 0; };
int g(D& d) { return d.f(); }
