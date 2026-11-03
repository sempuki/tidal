// tidal-rules: PFX-16
// tidal-expect: reject
// tidal-error-clang: cannot convert initializer list argument to .S.
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: cannot use a constructor marked .explicit.
// tidal-note: The argument is a braced list, and the only constructor that takes it is explicit.
struct S { explicit S(int, int) {} };
void f(S);
void g() { f({1, 2}); }
