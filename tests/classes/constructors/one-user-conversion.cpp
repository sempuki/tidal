// tidal-rules: CTOR-22
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .take
// tidal-error-gcc: could not convert .1. from .int. to .B
// tidal-error-edg: no suitable constructor exists to convert from .int. to .B
struct A { A(int) {} };
struct B { B(A) {} };
void take(B);
void f() { take(1); }
